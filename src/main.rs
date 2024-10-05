use dll_syringe::{
    process::{BorrowedProcess, OwnedProcess, ProcessModule},
    Syringe,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Anno 1800 Automation",
        options,
        Box::new(|_cc| Ok(Box::<Anno1800Automation>::default())),
    )
}

struct Anno1800Automation {
    status: String,
    syringe: Option<Syringe>,
    payload: Option<ProcessModule<BorrowedProcess<'static>>>,
}

impl Anno1800Automation {
    fn inject(&mut self) {
        let Some(process) = OwnedProcess::find_first_by_name("Anno1800.exe") else {
            self.status = "Anno 1800 not found".to_string();
            return;
        };

        self.syringe = Some(Syringe::for_process(process));

        let Some(syringe) = &self.syringe else {
            self.status = "Failed to create syringe".to_string();
            return;
        };

        let injected_payload = syringe.inject("Anno1800Automation.dll");
        self.payload = Some(injected_payload.unwrap());
    }

    fn eject(&mut self) {}
}

impl Default for Anno1800Automation {
    fn default() -> Self {
        Self {
            status: "Not injected".to_string(),
            syringe: None,
            payload: None,
        }
    }
}

impl eframe::App for Anno1800Automation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.status);
            if ui.button("Inject").clicked() {
                self.inject();
            }
            if ui.button("Eject").clicked() {
                self.eject();
            }
        });
    }
}
