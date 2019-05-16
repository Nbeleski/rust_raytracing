
    let nx: i32 = 200;
    let ny: i32 = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r: Ray = Ray(origin, lower_left_corner + u*horizontal + v*vertical);
            let col: Vec3 = color(r);
            let ir: i32 = (255.99*col.r()) as i32;
            let ig: i32 = (255.99*col.g()) as i32;
            let ib: i32 = (255.99*col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }  