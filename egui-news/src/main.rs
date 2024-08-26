use std::borrow::Cow;

use eframe::{
    egui::{
        CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout,
        ScrollArea, Separator, Vec2,
    },
    epi::App,
    run_native, NativeOptions,
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLUE: Color32 = Color32::from_rgb(0, 128, 255);

fn main() {
    let win_option = NativeOptions {
        initial_window_size: Some(Vec2::new(540.0, 960.0)),
        ..Default::default()
    };

    run_native(Box::new(Headlines::new()), win_option);
}

struct Headlines {
    articles: Vec<NewsCard>,
}

impl Headlines {
    fn new() -> Self {
        let iter = (0..20).map(|x| NewsCard {
            title: format!("Title {}", x),
            desc: format!("Description {}", x),
            url: format!("https://example.com/{}", x),
        });

        Self {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            String::from("Inter"),
            Cow::Borrowed(include_bytes!("../fonts/InterVariable.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.0),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.0),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, String::from("Inter"));

        ctx.set_fonts(font_def);
    }

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for x in &self.articles {
            ui.add_space(PADDING);

            let title = format!("{}", x.title);
            ui.colored_label(WHITE, title);

            ui.add_space(PADDING);
            let desc = Label::new(&x.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            ui.style_mut().visuals.hyperlink_color = BLUE;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&x.url).text("Read more ↗"));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "News"
    }
}

struct NewsCard {
    title: String,
    desc: String,
    url: String,
}
