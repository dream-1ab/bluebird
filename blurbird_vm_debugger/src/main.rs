use std::{cell::RefCell, fs::File, io::{Read, Write}, sync::Arc, vec};

use bluebird_vm::core::{ops::{Instructions, Op}, vm::{SysCallHandler, VirtualMachine}};
use eframe::{run_native, App, NativeOptions};
use egui::{mutex::Mutex, CentralPanel, Color32, Context, DragValue, Layout, ScrollArea, SidePanel, Slider, TextEdit, Ui, Vec2, Widget};


mod sample;

fn main() {
    run_native("bluebird_vm_debugger", NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(Vec2::new(1400f32, 1000f32)),
            ..Default::default()
        },
        ..NativeOptions::default()
    }, Box::new(|context| {
        Box::new(MyApp::new(context.egui_ctx.clone()))
    })).unwrap();
}


struct MyApp {
    source_code: String,
    vm: VirtualMachine<MySysCallHandler>,
    context: egui::Context,
    vm_run_config: Arc<Mutex<VmRunConfig>>,
}

#[derive(Clone, Debug)]
struct VmRunConfig {
    run_mode: VmRunMode,
    delay_time_ms: u32,
    cycle_count: u32,
    vm_run_signal: bool,
}

#[derive(Clone,Debug)]
enum VmRunMode {
    Manual,
    RunInDedicatedThreadLoop,
}

impl MyApp {
    fn get_vm_run_config(&self) -> VmRunConfig {
        let guard = self.vm_run_config.lock();
        guard.clone()
    }

    fn set_vm_run_config(&self, config: VmRunConfig) {
        let mut guard = self.vm_run_config.lock();
        *guard = config
    }
}

struct MySysCallHandler(Context, Arc<Mutex<VmRunConfig>>);
impl SysCallHandler for MySysCallHandler {
    fn handle(&mut self, syscall_number: u32) -> bool {
        match syscall_number {
            0 => { //println handler
                
                true
            }
            _ => false
        }
    }
}

impl MyApp {
    fn new(ctx: egui::Context) -> Self {
        let buffer = {
            let mut buffer = Vec::new();
            File::open("program.bb").unwrap().read_to_end(&mut buffer).unwrap();
            buffer
        };
        let vm_run_config = Arc::new(Mutex::new(VmRunConfig { run_mode: VmRunMode::Manual, delay_time_ms: 1000, cycle_count: 0, vm_run_signal: false }));
        let mut me = Self { 
            source_code: std::str::from_utf8(&buffer).unwrap().to_string(),
            vm: VirtualMachine::new(Instructions { ops: vec![] }, 128, MySysCallHandler(ctx.clone(), vm_run_config.clone())),
            context: ctx,
            vm_run_config: vm_run_config,
        };
        me.reinitialize_vm();
        me
    }

    fn reinitialize_vm(&mut self) {
        self.vm.reset();
        self.vm = VirtualMachine::new(Instructions::try_from(self.source_code.as_bytes()).unwrap(), 128, MySysCallHandler(self.vm.syscall_handler.0.clone(), self.vm.syscall_handler.1.clone()));
    }

    fn start_vm_loop(&mut self) {
        let vm_config = self.vm_run_config.clone();
        let ctx = self.context.clone();

        std::thread::spawn(move || {
            let mut counter = 0;
            loop {
                let vm_config = {
                    let mut guard = vm_config.lock();
                    guard.cycle_count = counter;
                    guard.vm_run_signal = true;
                    guard.clone()
                };
                if let VmRunMode::Manual = vm_config.run_mode {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(vm_config.delay_time_ms as u64));
                ctx.request_repaint();
                counter += 1;
            }
        });
    }

    fn register_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Registers");
            ui.separator();
            ui.columns(2, |uis| {
                let (left, right) = self.vm.comparison_register.values;

                uis[0].label("Program counter:");
                uis[1].label(format!("{} / {}", self.vm.ops.len(), self.vm.program_counter.program_counter));
                uis[0].label("End of program:");
                uis[1].label(format!("{}", self.vm.end_of_instruction()));
                uis[0].label("Comparison register:");
                uis[1].label(format!("{}, {}", left, right));
                uis[0].label("BEQ:");
                uis[1].label(format!("{}", left == right));
                uis[0].label("BNE:");
                uis[1].label(format!("{}", left != right));
                uis[0].label("BGT:");
                uis[1].label(format!("{}", left > right));
                uis[0].label("BLT:");
                uis[1].label(format!("{}", left < right));

                uis[0].label("Stack pointer:");
                uis[1].label(format!("{} / {}", self.vm.stack_memory.len(), self.vm.stack_pointer_register.stack_pointer));
            });
            ui.separator();
            ui.heading("Stack memory");
            
            for i in 0..self.vm.stack_memory.len() / 4 {
                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        if self.vm.stack_pointer_register.stack_pointer / 4 == i as u32 {
                            ui.visuals_mut().override_text_color = Some(Color32::GREEN);
                        }
                        ui.label(format!("{:04}", i * 4));
                    });
                    for a in 0..4 {
                        let addr = i * 4 + a;
                        ui.scope(|ui|{
                            if addr as u32 == self.vm.stack_pointer_register.stack_pointer {
                                ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::GREEN;
                            }
                            DragValue::new(&mut self.vm.stack_memory[addr]).ui(ui);
                        });
                    }
                    ui.label("=");
                    let mem_range = i * 4..i * 4 + 4;
                    let temp = &mut self.vm.stack_memory[mem_range.clone()];
                    let mut value = u32::from_le_bytes([temp[0], temp[1] ,temp[2] ,temp[3]]);
                    if ui.add(DragValue::new(&mut value)).changed() {
                        self.vm.stack_memory[mem_range].copy_from_slice(&mut value.to_le_bytes());
                    }
                });
            }
        });
    }

    fn instruction_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Instructions");
            ui.separator();

            {
                //Get vm run config
                let mut vm_run_config = self.get_vm_run_config();
                // println!("{:?}", vm_run_config);

                ui.horizontal_top(|ui| {
                    if ui.button("next ⏩").clicked() {
                        if !self.vm.end_of_instruction() {
                            self.vm.clock();
                        }
                        vm_run_config.run_mode = VmRunMode::Manual;
                        self.set_vm_run_config(vm_run_config.clone());
                    }
                    if ui.button("start 🔁").clicked() {
                        if let VmRunMode::Manual = &vm_run_config.run_mode {
                            vm_run_config.run_mode = VmRunMode::RunInDedicatedThreadLoop;
                            self.set_vm_run_config(vm_run_config.clone());
                            self.start_vm_loop();
                        }
                    }
                    if ui.button("stop 🔁").clicked() {
                        vm_run_config.run_mode = VmRunMode::Manual;
                        self.set_vm_run_config(vm_run_config.clone());
                    }
                    if ui.button("reset ⏹").clicked() {
                        self.vm.reset();
                    }
                });
                if Slider::new(&mut vm_run_config.delay_time_ms, 10..=1000).text("Clock delay ms").ui(ui).changed() {
                    self.set_vm_run_config(vm_run_config.clone());
                }
                ui.label(format!("Clock: {}", vm_run_config.cycle_count));

                if vm_run_config.vm_run_signal {
                    vm_run_config.vm_run_signal = false;
                    self.set_vm_run_config(vm_run_config);
                    if !self.vm.end_of_instruction() {
                        self.vm.clock();
                    }
                }
            }
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                for i in 0..self.vm.ops.len() {
                    let op = &self.vm.ops[i];
                    let mut op_text = op.to_string();
                    if let Op::Label(ref addr) = op {} else {
                        op_text.insert_str(0, "    ");
                    }
                    ui.scope(|ui| {
                        if i as u32 == self.vm.program_counter.program_counter {
                            ui.visuals_mut().override_text_color = Some(Color32::GREEN);
                        }
                        ui.label(format!("{:04}    {}", i, op_text));
                    });
                }
            })
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        SidePanel::right("vm debug panel").default_width(300f32).show(ctx, |ui| {
            self.register_panel(ctx, ui);
        });
        SidePanel::right("Instructions panel").show(ctx, |ui|{
            self.instruction_panel(ctx, ui);
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("كۆك قۇشقاچ virtual machine debugging tool.");
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("apply changes").clicked() {
                        {
                            let mut file = std::fs::OpenOptions::new().write(true).open("./program.bb").unwrap();
                            file.write_all(self.source_code.as_bytes()).unwrap();
                        }
                        self.reinitialize_vm();
                    }
                });
                ui.add_space(10f32);
                ScrollArea::vertical().show(ui, |ui| {
                    TextEdit::multiline(&mut self.source_code).code_editor().ui(ui);
                });
            })
        });
    }
}

