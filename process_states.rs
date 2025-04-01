use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProcessState {
    New,
    Ready,
    Running,
    Waiting,    // For I/O
    Terminated,
}

#[derive(Debug, Clone)]
struct Process {
    id: u32,
    state: ProcessState,
    cpu_time: u32,
    io_time: u32,
    total_cpu: u32,
    total_io: u32,
}

fn main() {
    let processes = Arc::new(Mutex::new(HashMap::new()));

    //Create some processes
    let mut handles = vec![];

    for i in 0..6 {
        let  processes_clone = Arc::clone(&processes);

        let handle = thread::spawn(move || {

            //Create process
            let mut proc = Process{
                id: i,
                state: ProcessState::New,
                cpu_time: 0,
                io_time: 0,
                total_cpu: 0,
                total_io: 0,
            };

            //Update Process to ready state
            {
                let mut procs = processes_clone.lock().unwrap();
                proc.state = ProcessState::Ready;
                procs.insert(proc.id, proc.clone());
                println!("Process {}: {:?}", proc.id, proc.state);
            }

            //Simulate process lifecycle
            for _ in 0..10 {

                //Scheduled to run (Ready -> Running)
                let mut procs = processes_clone.lock().unwrap();
                proc.state = ProcessState::Running;
                procs.insert(proc.id, proc.clone());
                println!("Process {}: {:?}", proc.id, proc.state);
            }

            //Simulate CPU Work
            let cpu_work = Duration::from_millis(500 + (i + 20) as u64);
            let start = Instant::now();
            while start.elapsed() < cpu_work {
                //A simple Computation
                let _ = 12345 * 65432;
            }
            proc.cpu_time += cpu_work.as_millis() as u32;
            proc.total_cpu += cpu_work.as_millis() as u32;

            //I/O wait (Running -> Waiting)
            {
                let mut procs = processes_clone.lock().unwrap();
                proc.state = ProcessState::Waiting;
                procs.insert(proc.id, proc.clone());
                println!("Process {}: {:?} (CPU time: {}ms)",
                         proc.id, proc.state, cpu_work.as_millis());
            }

            //  I/O operation
            let io_wait = Duration::from_millis(300 + (i * 100) as u64);
            thread::sleep(io_wait);
            proc.io_time += io_wait.as_millis() as u32;
            proc.total_io += io_wait.as_millis() as u32;

            // I/O complete (Waiting -> Ready)
            {
                let mut procs = processes_clone.lock().unwrap();
                proc.state = ProcessState::Ready;
                procs.insert(proc.id, proc.clone());
                println!("Process {}: {:?} (I/O time: {}ms)",
                         proc.id, proc.state, io_wait.as_millis());
            }

            // Terminate process
            {
                let mut procs = processes_clone.lock().unwrap();
                proc.state = ProcessState::Terminated;
                procs.insert(proc.id, proc.clone());
                println!("Process {}: {:?} (Total CPU: {}ms, Total I/O: {}ms)",
                         proc.id, proc.state, proc.total_cpu, proc.total_io);
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("\nFinal Process States:");
    let procs = processes.lock().unwrap();
    for (id, proc) in procs.iter() {
        println!("Process {}: {:?}, CPU: {}ms, I/O: {}ms",
                 id, proc.state, proc.total_cpu, proc.total_io);
    }
}