# rustbasedTests
rustbasedTests

Введённые изменения:
- произведён полный рефакторинх

- пример

<details>

  ```
    pub trait Controller {
    fn position(&self) -> Vec3f;
    fn set_position(&mut self, pos: Vec3f);
    fn forward(&self) -> Vec3f;
    fn set_forward(&mut self, forward: Vec3f);
    fn velocity(&self) -> Vec3f;
    fn set_velocity(&mut self, vel: Vec3f);
    fn set_grounded(&mut self, grounded: bool);
    fn is_grounded(&self) -> bool;
    fn walk_speed(&self) -> f32;
    fn jump_power(&self) -> f32;
    fn gravity(&self) -> f32;
    fn right(&self) -> Vec3f;
    fn is_moving(&self) -> bool;
    fn set_is_moving(&mut self, val: bool);
}
#[rustfmt::skip]
impl Controller for Camera {
    fn position(&self) -> Vec3f                 { self.position }
    fn set_position(&mut self, pos: Vec3f)      { self.position = pos; }
    fn forward(&self) -> Vec3f                  { self.forward }
    fn set_forward(&mut self,forward: Vec3f)    { self.forward = forward; }
    fn velocity(&self) -> Vec3f                 { self.velocity }
    fn set_velocity(&mut self, vel: Vec3f)      { self.velocity = vel; }
    fn set_grounded(&mut self, grounded: bool)  { self.is_grounded = grounded; }
    fn is_grounded(&self) -> bool               { self.is_grounded }
    fn walk_speed(&self) -> f32                 { self.walk_speed}
    fn jump_power(&self) -> f32                 { self.jump_power}
    fn gravity(&self) -> f32                    { self.gravity}
    fn right(&self) -> Vec3f                    { self.forward().cross(Vec3f::new(0.0, 1.0, 0.0)).normalize()}
    fn is_moving(&self) -> bool                 { self.is_moving}
    fn set_is_moving(&mut self,val:bool)        { self.is_moving=val;}
}
pub fn input_controller<T: Controller>(/*args*/){
//logic
}

в цикле до отрисовки
input_controller(&mut camera, &keyboard_state,&mut world,selected_brush);//<- 1 текущий интерфейс 
if camera.no_clip {
    camera.update_position(&keyboard_state, delta_time);//дефолтовое движение камеры не учитывая физики
    } else {
      collide_controller(&mut camera, &world);//<- 1 интерфейс для физики
    }
}
```

</details>

---
- произведена оптимизация отрисовки
- улучшено использование функционала отрисовки и коллизий
- частички теперь сталкиваются с миром :)

<img width="1852" height="983" alt="image" src="https://github.com/user-attachments/assets/c6465a21-290c-4fe2-afa9-25e02d6f54f7" />
