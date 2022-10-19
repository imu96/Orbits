use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    _wind: window::Id,
}

fn model(_app: &App) -> Model {
    let _wind = _app.new_window().size(1500,1110).title("aoeu")
	.view(orbits).build().unwrap();
    Model {_wind}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn orbits(app: &App, _model: &Model, frame: Frame){
    // Prepare to draw.
    let draw = app.draw();
    // Get current time
    let tt = app.elapsed_frames() as f32;
    let tt2 = app.elapsed_frames() ;
    let dot_col = CORNFLOWERBLUE;
    let line_col = STEELBLUE;
    let sz = 10.0;
    let mults = [(158.3, 250.0), (3333.3, 555.0)];

    // plot dots and stores their positions to plot lines
    let mut posns = Vec::new();
    for (vel, dist) in mults {
	let xx = dist * ((tt/vel).sin());
	let yy = dist * ((tt/vel).cos());
	draw.ellipse().color(dot_col).w(sz).h(sz)
	    .x_y(xx,yy);
	if tt2 % 10 == 0 {
	    posns.push(Vec2::new(xx,yy));
	}
    }

    for ii in 1..posns.len() {
	draw.line().start(posns[ii]).end(posns[ii-1])
	    .weight(2.0).color(line_col);
    }

    // Clear the background to black
    // if tt2 < 2{
    if tt2 < 2{
	draw.background().color(BLACK);
    }

    let mut path = String::from("/home/imran/Pictures/nannou/take1/");
    let tt3 = format!("{:05}",tt2);
    path = path + &tt3;
    path.push_str(".png");
    app.window(app.window_id()).unwrap().capture_frame(path);

    draw.to_frame(app, &frame).unwrap();
}
