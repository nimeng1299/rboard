use std::{
    io::{BufRead, BufReader, Write},
    sync::{Arc, Mutex},
    thread,
};

use iced::futures::{SinkExt, executor::block_on};
use subprocess::{Exec, Popen, PopenError, Redirection};

pub struct GTP {
    cmd_tx: std::sync::mpsc::Sender<String>,
    child: Popen,
    cmd_handler: Option<thread::JoinHandle<()>>,
    output_handler: Option<thread::JoinHandle<()>>,
}

impl GTP {
    pub fn start(
        engine_path: &str,
        engine_args: &str,
        data_tx: Arc<Mutex<iced::futures::channel::mpsc::Sender<String>>>,
    ) -> Result<Self, String> {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<String>();

        let mut child = spawn_child_process(engine_path, engine_args)
            .map_err(|err| format!("无法启动进程: {}", err))?;
        debug(format!("子进程已启动 (PID: {})", child.pid().unwrap_or(0)));

        let mut stdin = child
            .stdin
            .take()
            .ok_or("无法获取子进程的标准输入".to_string())?;
        let stdout = child
            .stdout
            .take()
            .ok_or("无法获取子进程的标准输出")
            .unwrap();
        let stderr = child
            .stderr
            .take()
            .ok_or("无法获取子进程的标准错误")
            .unwrap();

        // 启动命令发送线程
        let cmd_handler = thread::spawn(move || {
            for command in cmd_rx {
                if command == "rboard gtp exit".to_string() {
                    break;
                }
                if let Err(e) = stdin.write_all(format!("{}\n", command).as_bytes()) {
                    eprintln!("写入命令失败: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush() {
                    eprintln!("刷新输入失败: {}", e);
                    break;
                }
            }
            eprintln!("命令线程已退出");
        });

        let data_tx_clone = Arc::clone(&data_tx);
        // 启动输出读取线程
        let output_handler = thread::spawn(move || {
            let data_tx_out = Arc::clone(&data_tx_clone);
            let data_tx_err = Arc::clone(&data_tx_clone);
            // 使用两个线程分别读取 stdout 和 stderr
            let stdout_thread = thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(output) => {
                            let _ = block_on(data_tx_out.lock().unwrap().send(output));
                        }

                        Err(e) => edebug(format!("读取输出错误: {}", e)),
                    }
                }
            });

            let stderr_thread = thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    match line {
                        Ok(err) => {
                            let _ = block_on(data_tx_err.lock().unwrap().send(err));
                        }
                        Err(e) => edebug(format!("读取错误输出错误: {}", e)),
                    }
                }
            });

            // 等待两个线程完成
            stdout_thread.join().expect("stdout 线程崩溃");
            stderr_thread.join().expect("stderr 线程崩溃");
        });

        Ok(GTP {
            cmd_tx,
            child,
            cmd_handler: Some(cmd_handler),
            output_handler: Some(output_handler),
        })
    }

    pub fn send_command(&self, command: String) -> Result<(), String> {
        self.cmd_tx
            .send(command.trim().to_string())
            .map_err(|e| e.to_string())
    }

    pub fn send_kata_analyze(&self) -> Result<(), String> {
        self.send_command("kata-analyze 15 pvVisits true".to_string())
    }

    pub fn exit(&mut self) -> Result<(), String> {
        let _ = self
            .cmd_tx
            .send("rboard gtp exit".to_string())
            .map_err(|e| e.to_string());
        if let Some(_) = self.child.poll() {
            debug("子进程已退出".to_string());
        } else {
            debug("终止子进程...".to_string());
            if let Err(e) = self.child.terminate() {
                edebug(format!("终止子进程失败: {}", e));
            }
        }

        let _ = self.child.kill();

        if let Some(handler) = self.cmd_handler.take() {
            debug("终止子进程 cmd_handler ...".to_string());
            let _ = handler.join();
            debug("终止子进程 cmd_handler Ok...".to_string());
        }

        if let Some(handler) = self.output_handler.take() {
            debug("终止子进程 output_handler ...".to_string());
            let _ = handler.join();
            debug("终止子进程 output_handler Ok...".to_string());
        }
        Ok(())
    }
}

impl Drop for GTP {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

fn debug(msg: String) {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("[D] {} GTP Engine: {}", current_time, msg);
}

fn edebug(msg: String) {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    eprintln!("[E] {} GTP Engine: {}", current_time, msg);
}

fn spawn_child_process(engine_path: &str, engine_args: &str) -> Result<Popen, PopenError> {
    let process = Exec::cmd(engine_path)
        .args(&parse_args(&engine_args.to_string()))
        .stdin(Redirection::Pipe) // 管道输入
        .stdout(Redirection::Pipe) // 管道输出
        .stderr(Redirection::Pipe) // 管道错误
        .popen();

    if let Ok(p) = &process {
        if let Some(pid) = p.pid() {
            println!("子进程 PID: {}", pid);
        }
    }

    process
}

fn parse_args<'a>(args: &'a String) -> Vec<&'a str> {
    let a: Vec<&'a str> = args.split_whitespace().collect();
    a
}
