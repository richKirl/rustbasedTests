mod math;
mod shader;
use math::mat4vf::Mat4vf;
use math::quatf::Quatf;
use math::vec3f::Vec3f;
use shader::shader::Shader;
use shader::shader::Uniform;
use shader::shaders;
extern crate gl;
extern crate sdl2;

fn main() {
    // 1. Инициализация SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // 2. Настройка параметров OpenGL (версия 4.6 Core)
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    // 3. Создание окна
    let window = video_subsystem
        .window("Rust 3D Math Engine", 800, 600)
        .opengl() // Важно: разрешает создание GL-контекста
        .resizable()
        .build()
        .unwrap();

    // 4. Создание контекста и загрузка функций
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // 5. Главный цикл
    let mut event_pump = sdl_context.event_pump().unwrap();
    let timer = sdl_context.timer().unwrap();
    let vertices: [Vec3f; 3] = [
        Vec3f::new(-0.5, -0.5, 0.0), // Лево низ
        Vec3f::new(0.5, -0.5, 0.0),  // Право низ
        Vec3f::new(0.0, 0.5, 0.0),   // Верх центр
    ];

    let (mut vbo, mut vao) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Описываем атрибут 0 (Position)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }
    let mut shader_main = Shader::new(shaders::VS_SRC, shaders::FS_SRC);
    shader_main.set_loc(Uniform::Mvp, 5);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Здесь будем использовать вашу математику
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            shader_main.use_id();
            // 1. Получаем время в секундах
            let ticks = timer.ticks() as f32 / 1000.0;

            // 2. Создаем кватернион поворота (например, вокруг оси Y)
            let axis = Vec3f::new(0.0, 1.0, 0.0);
            let rotation_quat = Quatf::from_axis_angle(axis, ticks); // вращается со скоростью 1 рад/сек

            // 3. Превращаем кватернион в матрицу модели (используя ваш impl From<Quatf>)
            let model: Mat4vf = rotation_quat.into();

            // 4. Настраиваем камеру (View) и проекцию (Proj)
            let view = Mat4vf::look_at(
                Vec3f::new(0.0, 0.0, 3.0), // Камера отъехала назад на 3 единицы
                Vec3f::new(0.0, 0.0, 0.0), // Смотрит в центр
                Vec3f::new(0.0, 1.0, 0.0), // Верх — это Y
            );

            let proj = Mat4vf::perspective(45.0f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);

            // 5. Итоговая матрица MVP (порядок: Projection * View * Model)
            let mvp = proj * view * model;

            // 6. Отправляем в шейдер через ваш метод
            shader_main.set_mat4x4(Uniform::Mvp, &mvp);
            // gl::UseProgram(shader_program); // Здесь должен быть ID вашего шейдера
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window(); // Отрисовка кадра
    }
}
