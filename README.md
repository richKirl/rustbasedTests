# rustbasedTests
rustbasedTests

<img width="1852" height="1052" alt="image" src="https://github.com/user-attachments/assets/9ebc0aaa-ea22-4a4b-9865-d2390cb708a6" />


Введённые изменения:

- произведен полный профайлинх исходного кода программы, теперь всё считается мега быстро,

общие состояния стабильные по возможности теперь ничего не фонит и не просачивается на сколько это возможно

<details>

```
--------------------------------------------------------------------------------
-- Metadata
--------------------------------------------------------------------------------
Invocation:       /usr/bin/cg_annotate cachegrind.out.39578
Command:          target/release/math3d
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Threshold:        0.1%
Annotation:       on

--------------------------------------------------------------------------------
-- Summary
--------------------------------------------------------------------------------
Ir____________________ 

3,295,970,112 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
-- File:function summary
--------------------------------------------------------------------------------
  Ir__________________________  file:function

< 3,198,728,671 (97.0%, 97.0%)  ???:
  2,898,149,272 (87.9%)           math3d::main
    279,442,870  (8.5%)           ???

<     9,092,844  (0.3%, 97.3%)  ./libio/./libio/getc.c:getc

<     8,641,030  (0.3%, 97.6%)  ./nptl/./nptl/pthread_mutex_lock.c:pthread_mutex_lock@@GLIBC_2.2.5

<     8,358,526  (0.3%, 97.8%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:
      8,062,957  (0.2%)           __memcpy_avx_unaligned_erms

<     7,080,067  (0.2%, 98.1%)  ./string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S:
      7,079,899  (0.2%)           __memset_avx2_unaligned_erms

<     6,740,743  (0.2%, 98.3%)  ./nptl/./nptl/pthread_mutex_unlock.c:pthread_mutex_unlock@@GLIBC_2.2.5

<     5,770,730  (0.2%, 98.4%)  ./string/../sysdeps/x86_64/multiarch/strcmp-avx2.S:
      4,521,264  (0.1%)           __strcmp_avx2

--------------------------------------------------------------------------------
-- Function:file summary
--------------------------------------------------------------------------------
  Ir__________________________  function:file

> 2,898,149,272 (87.9%, 87.9%)  math3d::main:???

>   279,452,258  (8.5%, 96.4%)  ???:
    279,442,870  (8.5%)           ???

>     9,092,844  (0.3%, 96.7%)  getc:./libio/./libio/getc.c

>     8,641,030  (0.3%, 96.9%)  pthread_mutex_lock@@GLIBC_2.2.5:./nptl/./nptl/pthread_mutex_lock.c

>     8,062,957  (0.2%, 97.2%)  __memcpy_avx_unaligned_erms:./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S

>     7,079,899  (0.2%, 97.4%)  __memset_avx2_unaligned_erms:./string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S

>     6,740,743  (0.2%, 97.6%)  pthread_mutex_unlock@@GLIBC_2.2.5:./nptl/./nptl/pthread_mutex_unlock.c

>     4,521,264  (0.1%, 97.7%)  __strcmp_avx2:./string/../sysdeps/x86_64/multiarch/strcmp-avx2.S

...
```

</details>

код

<details>

```
find . -type f -name "*.rs" -exec wc -l -c {} +
    63   2106 ./window/win.rs
     1     13 ./window/mod.rs
   150   5667 ./main.rs
   261  10407 ./geometry/texturerel.rs
    26    542 ./geometry/light.rs
    99   3307 ./geometry/light_buffers.rs
   230   8071 ./geometry/particle.rs
   106   3752 ./geometry/camera.rs
     6    106 ./geometry/mod.rs
  1111  38618 ./geometry/util.rs
   555  19993 ./shader/shaders.rs
     2     33 ./shader/mod.rs
   127   3943 ./shader/shader.rs
    21    815 ./initialize/initshaders.rs
    78   5145 ./initialize/initmaplight.rs
     2     43 ./initialize/mod.rs
   124   4829 ./events/eventmanager.rs
     1     22 ./events/mod.rs
   224   8206 ./physics/inputcontroller.rs
     2     52 ./physics/mod.rs
   123   4408 ./physics/collidecontroller.rs
   223   6119 ./math/quatf.rs
   223   5488 ./math/vec3d.rs
   122   4284 ./math/mat4vf.rs
   242   6017 ./math/vec4f.rs
   244   6032 ./math/vec3f.rs
   226   5534 ./math/vec4d.rs
   130   4549 ./math/frustum.rs
     9    145 ./math/mod.rs
   196   7278 ./math/dualquatf.rs
  4927 165524 total


```

</details>

--

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
