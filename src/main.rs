use yew::prelude::*;
use web_sys::HtmlInputElement;

// Define the properties that the CameraSimulator component will accept
#[derive(Properties, PartialEq)]
struct CameraProps {
    pub initial_zoom: f32,
    pub initial_brightness: f32,
}

struct CameraSimulator {
    zoom: f32,
    brightness: f32,
    focus_mode: bool,
    cam_value: i32,
    //video_ref : NodeRef //Reference to video element
}

pub enum Msg {
    ZoomIn,
    ZoomOut,
    IncreaseBrightness,
    DecreaseBrightness,
    ChangeFocusMode,
    SetValue(i32)
}

impl Component for CameraSimulator {
    type Message = Msg;
    type Properties = CameraProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            zoom: ctx.props().initial_zoom,  // Use initial zoom level from properties
            brightness: ctx.props().initial_brightness,  // Use initial brightness from properties
            focus_mode: false,
            cam_value: 0,
            //video_ref: NodeRef::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ZoomIn => {
                self.zoom = (self.zoom + 0.1).min(3.0); // Max zoom 3x
            }
            Msg::ZoomOut => {
                self.zoom = (self.zoom - 0.1).max(1.0); // Min zoom 1x
            }
            Msg::IncreaseBrightness => {
                self.brightness = (self.brightness + 0.1).min(2.0); // Max brightness 2x
            }
            Msg::DecreaseBrightness => {
                self.brightness = (self.brightness - 0.1).max(0.5); // Min brightness 0.5x
            }
            Msg::ChangeFocusMode => {
                self.focus_mode = !self.focus_mode;
            }
            Msg::SetValue(val) => {
                self.cam_value = val;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let zoom_in = ctx.link().callback(|_| Msg::ZoomIn);
        let zoom_out = ctx.link().callback(|_| Msg::ZoomOut);
        let inc_brightness = ctx.link().callback(|_| Msg::IncreaseBrightness);
        let dec_brightness = ctx.link().callback(|_| Msg::DecreaseBrightness);
        let change_focus_mode = ctx.link().callback(|_| Msg::ChangeFocusMode);

        let label_focusmode = if self.focus_mode { "Automatic" } else { "Manual" };
        let toggle_class_focusmode = if self.focus_mode { "toggle-on" } else { "toggle-off" };

        let on_camvalue_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_i32 = input.value().parse::<i32>().unwrap();
            Msg::SetValue(input_i32)
        });

        html! {
            <div class="container">
                <h1>{ "Camera Simulator" }</h1>
                
                <div class="control-camera-container">
                    <div class="controls">
                        <button onclick={zoom_in} class="button">{ "Zoom In" }</button>
                        <button onclick={zoom_out} class="button">{ "Zoom Out" }</button>
                        <button onclick={inc_brightness} class="button">{ "Increase Brightness" }</button>
                        <button onclick={dec_brightness} class="button">{ "Decrease Brightness" }</button>
        
                        <div class="focus-toggle">
                            <label>{ "Focus Mode:" }</label>
                            <div class="toggle-container" onclick={change_focus_mode}> 
                                <div class={classes!("toggle-button", toggle_class_focusmode)}> { label_focusmode } </div>
                            </div>
                        </div>
        
                        <div class="focus-toggle">
                            <label>{ "Value:" }</label>
                            <input
                                type="number"
                                class="input"
                                min="0" max="5"
                                value={self.cam_value.to_string()}
                                oninput={on_camvalue_input}
                                placeholder="Cam Value"
                            />
                        </div>
                    </div>
        
                    <div class="camera-view" style={format!("transform: scale({}); filter: brightness({});", self.zoom, self.brightness)}>
                        { "Camera View" }
                    </div>
                </div>
        
                <div class="status">
                    <p>{ format!("Current Zoom: {:.1}x", self.zoom) }</p>
                    <p>{ format!("Current Brightness: {:.1}", self.brightness) }</p>
                    <p>{ format!("Current Mode: {}", label_focusmode) }</p>
                    <p>{ format!("Cam Value: {}", self.cam_value) }</p>
                </div>
            </div>
        }

    }
}

// Root component that instantiates the CameraSimulator with properties
struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{ "Camera App" }</h2>
                /*  Passing initial zoom and brightness via props */
                <CameraSimulator initial_zoom={1.2} initial_brightness={1.0} />
            </div>
        }
    }
}

fn main() {

    yew::Renderer::<App>::new().render();

}
