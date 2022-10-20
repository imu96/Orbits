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
    let _wind = _app.new_window().size(1500,1200)
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

    // Size of dots
    let sz = 10.0;
     
    /* Each tuple consists of: 
    1) velocity multiplier
    2) orbital distance from central point
    3) offset from origin (can be used for x or y direction)
    4) elliptical multiplier for stretching in the y direction
    5) elliptical multiplier for stretching in the x direction */
    let params = [(78.3, 250.0, 0.0, 1.0, 0.5),
		  (1333.3, 505.0, 0.0, 0.5, 1.0)];

    // Draw dots and store their positions to draw lines
    let mut posns = Vec::new();
    for (vel, dist, offset, ell1, ell2) in params {
	let xx = (dist * ((tt/vel).sin())) * ell1 +offset;
	let yy = dist * ((tt/vel).cos()) * ell2;
	draw.ellipse().color(dot_col).w(sz).h(sz)
	    .x_y(xx,yy);
	if tt2 % 10 == 0 {
	    posns.push(Vec2::new(xx,yy));
	}
    }

    // Draw lines
    for ii in 1..posns.len() {
	draw.line().start(posns[ii]).end(posns[ii-1])
	    .weight(2.0).color(line_col);
    }

    // Clear the background to black
    if tt2 < 2{
	draw.background().color(BLACK);
    }

    // Functionality for outputting frames to png in order
    // to make an mp4
    // let mut path = String::from("/home/imran/Pictures/nannou/take1/");
    // let tt3 = format!("{:05}",tt2);
    // path = path + &tt3;
    // path.push_str(".png");
    // app.window(app.window_id()).unwrap().capture_frame(path);

    draw.to_frame(app, &frame).unwrap();
}
