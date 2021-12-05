

use eframe::{epi, egui};
use eframe::egui::{
    plot::Plot,
    plot::Line,
    plot::Legend,
    plot::Values,
    plot::LineStyle,
    Color32
};
use core::time::Duration;
use std::io;
use std::convert::TryInto;
use std::io::Write;
use std::sync::mpsc::Receiver;

struct MyEguiApp {
    value_channel: Receiver<u16>,
    saved_values: Vec<f64>,
    display_offset: usize
}

impl epi::App for MyEguiApp {
   fn name(&self) -> &str {
       "My egui App"
   }

   fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {

        println!("Updating, last: {}",  self.saved_values[self.display_offset as usize]);
        while let Ok(value) = self.value_channel.try_recv(){
            self.saved_values[self.display_offset as usize] = value as f64 / 4096.0;
            self.display_offset = ( self.display_offset + 1) %self.saved_values.len();
        }


       egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let mut plot = Plot::new("lines_demo")
                .legend(Legend::default())
                .include_x(0)
                .include_x(512)
                .data_aspect(512f32/1f32);

            let values = self.saved_values.clone(); 
            let values2 = self.saved_values.clone(); 
            let display_offset = self.display_offset;

            let line = plot
            .line(
                Line::new(Values::from_explicit_callback(
                    // move |x| 0.5 * (2.0 * x).sin(),
                    move |x| values[((x as usize))%values.len()],
                    (0.0..512.0),
                    512,
                ))
                .color(Color32::from_rgb(200, 100, 100))
                .style(LineStyle::Solid)
                .name("wave")
            )
            .line(
                Line::new(Values::from_explicit_callback(
                    // move |x| 0.5 * (2.0 * x).sin(),
                    move |x| {
                        let mut agg = vec![0.0;8];
                        for i in 0..8{
                            agg[i] = values2[((x as usize + i as usize))%values2.len()];
                        }
                        agg.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let res = (agg[4] + agg[5])/2.0;
                        res
                    },
                    (0.0..512.0),
                    64,
                ))
                .color(Color32::from_rgb(100, 100, 200))
                .style(LineStyle::Solid)
                .name("wave_agg")
            );
            ui.add(line);
       });

       ctx.request_repaint();

   }
}

fn main() {
    let (tx,rx) = std::sync::mpsc::channel();
    let app = MyEguiApp{
        value_channel: rx,
        saved_values: vec![0.5;512],
        display_offset: 0
    };

    let native_options = eframe::NativeOptions::default();



    std::thread::spawn(move ||{
        let ports = serialport::available_ports().expect("No ports found!");
        for p in ports {
            println!("{}", p.port_name);
        }
    
        let mut port = serialport::new("/dev/ttyACM0", 115_200)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port");
    
        let mut serial_buf: Vec<u8> = vec![0; 2];
        loop{
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let (int_bytes, rest) = serial_buf.split_at(std::mem::size_of::<u16>());
                    let res = u16::from_le_bytes(int_bytes.try_into().unwrap());
                    // println!("received {}", res);
                    if res != 0{
                        tx.send(res);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
    
        }
    });


    eframe::run_native(Box::new(app), native_options);




    


}