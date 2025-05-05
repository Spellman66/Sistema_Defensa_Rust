use eframe::{egui, App, CreationContext, Frame};
use sysinfo::{System, SystemExt, CpuExt};  // Añadimos CpuExt aquí

struct DefensaApp {
    system: System,
    logs: Vec<String>,
}

impl DefensaApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            system: sys,
            logs: vec!["- Sistema iniciado".to_string()],
        }
    }
}

impl App for DefensaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.system.refresh_all();

        let cpu_usage = self.system.global_cpu_info().cpu_usage(); // Ya funciona con CpuExt
        let mem_total = self.system.total_memory();
        let mem_used = self.system.used_memory();
        let mem_pct = mem_used as f32 / mem_total as f32 * 100.0;

        if cpu_usage > 85.0 {
            self.logs.push(format!("⚠️ CPU Alta: {:.2}%", cpu_usage));
        }
        if mem_pct > 85.0 {
            self.logs.push(format!("⚠️ RAM Alta: {:.2}%", mem_pct));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🛡 Sistema de Defensa");
            ui.label(format!("CPU: {:.2}%", cpu_usage));
            ui.label(format!("RAM: {:.2}%", mem_pct));
            ui.separator();
            ui.label("Logs recientes:");
            for log in self.logs.iter().rev().take(5) {
                ui.label(log);
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sistema de Defensa",
        options,
        Box::new(|cc| Box::new(DefensaApp::new(cc))),
    )
}