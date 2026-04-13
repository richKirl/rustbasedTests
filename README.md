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
Инфо:

```

find . -type f -name "*.rs" -exec wc -l -c {} +
   334  17065 ./main.rs
   261  10407 ./geometry/texturerel.rs
    26    542 ./geometry/light.rs
    99   3307 ./geometry/light_buffers.rs
   225   7928 ./geometry/particle.rs
   103   3519 ./geometry/camera.rs
     6    106 ./geometry/mod.rs
  1167  41837 ./geometry/util.rs
   555  19993 ./shader/shaders.rs
     2     33 ./shader/mod.rs
   127   3943 ./shader/shader.rs
   154   5542 ./physics/inputcontroller.rs
     2     52 ./physics/mod.rs
   123   4408 ./physics/collidecontroller.rs
   223   6119 ./math/quatf.rs
   223   5488 ./math/vec3d.rs
   122   4284 ./math/mat4vf.rs
   242   6017 ./math/vec4f.rs
   241   5920 ./math/vec3f.rs
   226   5534 ./math/vec4d.rs
   130   4549 ./math/frustum.rs
     9    145 ./math/mod.rs
   196   7278 ./math/dualquatf.rs
  4796 164016 total

```

</details>

---
- произведена оптимизация отрисовки
- улучшено использование функционала отрисовки и коллизий
- частички теперь сталкиваются с миром :)

<details>

<img width="1852" height="983" alt="image" src="https://github.com/user-attachments/assets/c6465a21-290c-4fe2-afa9-25e02d6f54f7" />

</details>
