pub const VS_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 5) uniform mat4 mvp; // Одна матрица для всего

void main() {
    gl_Position = mvp * vec4(aPos, 1.0);
}
"#;

//simple fragment shader for cube
pub const FS_SRC: &str = r#"
#version 460 core
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0); // Оранжевый цвет
}
"#;

// pub const VSWC_SRC: &str = r#"
// #version 460 core
// layout (location = 0) in vec3 a_Pos;
// layout (location = 5)uniform mat4 model;
// layout (location = 6)uniform mat4 view;
// layout (location = 7)uniform mat4 projection;

// void main(){
//     gl_Position = projection * view * model * vec4(a_Pos, 1.0);
// }
// "#;

// //simple fragment shader for cube
// pub const FSWC_SRC: &str = r#"
// #version 460 core
// out vec4 out_Color;
// void main(){
//     out_Color = vec4(1.0, 1.0, 1.0, 0.3);
// }
// "#;
// //cube particles
// pub const VSPC_SRC: &str = r#"
// #version 460 core
// layout (location = 0) in vec3 aPos;      // Позиция
// layout (location = 1) in vec4 aInstance; // (x, y, z, tex_id)
// layout (location = 2) in vec2 aUV;       // Получаем UV из буфера

// // layout (location = 5)uniform mat4 model;
// layout (location = 6)uniform mat4 view;
// layout (location = 7)uniform mat4 projection;

// out float v_TexID;
// out vec2 uv;

// void main() {
//     uv = aUV;
//     v_TexID = aInstance.w;

//     // Позиция инстанса + локальная позиция
//     vec3 worldPos = aPos + aInstance.xyz;
//     gl_Position = projection * view * vec4(worldPos, 1.0);
// }
// "#;
// //simple fragment shader for cubeplayer particles
// pub const FSPC_SRC: &str = r#"
// #version 460 core
// // Получаем из вершинного шейдера
// in float v_TexID;
// // Если ты добавил в вершины частиц нормали или освещение:
// in float v_AO;
// in vec2 uv;
// out vec4 FragColor;

// // Твой массив текстур (тот же, что и для чанков)
// layout (location = 8)uniform sampler2DArray u_TextureArray;

// void main() {
//     // В частицах обычно не используют сложные UV (0..1),
//     // так как частица — это просто "осколок" цвета.
//     // Поэтому можно просто взять центральный пиксель текстуры блока.
//     //vec3 uv = vec3(0.0, 1.0, v_TexID);

//     vec4 texColor = texture(u_TextureArray, vec3(uv,v_TexID));

//     // Если цвет текстуры слишком прозрачный (например, для листвы),
//     // выбрасываем пиксель
//     // if (texColor.a < 0.1) {
//     //     discard;
//     // }

//     // Применяем небольшое затемнение, чтобы частицы выглядели объемнее
//     // (можно умножить на v_AO, если ты передаешь его)
//     FragColor = texColor;
// }
// "#;
// //cursor
// pub const VSDC_SRC: &str = r#"
// #version 460 core
// layout (location = 0) in vec3 aPos;
// layout (location = 1) in vec2 aTexCoord;

// out vec2 TexCoord;
// out float vLayer;
// flat out int flag;
// // u_Projection теперь будет ортографической матрицей
// layout (location = 5)uniform mat4 model;
// // layout (location = 6)uniform mat4 view;
// layout (location = 7)uniform mat4 projection;
// layout (location = 12)uniform int layer;
// layout (location = 11)uniform int useArray;
// void main() {
//     TexCoord = aTexCoord;
//     vLayer=float(layer);
//     flag=useArray;
//     // Умножаем позицию на ортографическую матрицу
//     gl_Position = projection *model* vec4(aPos, 1.0);
// }
// "#;
// //cursor
// //simple fragment shader for cube
// pub const FSDC_SRC: &str = r#"
// #version 460 core
// out vec4 FragColor;
// in vec2 TexCoord;
// in float vLayer;
// flat in int flag;
// layout (location = 8)uniform sampler2DArray u_TextureArray;
// layout (location = 9)uniform sampler2D texture1;

// // layout (location = 10)uniform vec4 u_Color;

// void main() {
// if (flag == 1) {
// vec4 texColor = texture(u_TextureArray, vec3(1-TexCoord,vLayer ));
// FragColor = texColor * vec4(1,1,1,1);
// }
// else if (flag == 0){
//     vec4 texColor = texture(texture1, 1-TexCoord);
//     FragColor = texColor * vec4(1,1,1,1);
//     }
// }
// "#;

pub const VSTC_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
// u_Projection теперь будет ортографической матрицей
layout (location = 5)uniform mat4 model;
// layout (location = 6)uniform mat4 view;
layout (location = 7)uniform mat4 projection;
void main() {
    TexCoord = aTexCoord;
    // Умножаем позицию на ортографическую матрицу
    gl_Position = projection *model* vec4(aPos, 1.0);
}
"#;

pub const FSTC_SRC: &str = r#"
#version 460 core
in vec2 TexCoord;
out vec4 FragColor;
layout (location = 9)uniform sampler2D textTexture;

void main() {
vec4 sampled = vec4(1.0, 1.0, 1.0, texture(textTexture, TexCoord).r);
FragColor = vec4(vec3(1.0, 1.0, 1.0), 1.0) * sampled;
}
"#;

pub const VSDDC_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 aPos;     // Базовый квадрат [0..1]
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec2 iOffset;  // Позиция в пикселях (X, Y)
layout (location = 3) in vec2 iScale;   // Размер в пикселях (W, H)
layout (location = 4) in float iTexID;  // Индекс в атласе 16x16
layout (location = 5) in float iAtlas;  // Индекс atlas 0 - blocks 1 - uiicons
layout (location = 7)uniform mat4 projection;//uniform mat4 projection; //матрица glm::ortho(0, width, height, 0)

out vec2 TexCoord;
out float TexIndex;
out float Index;
void main() {
    TexCoord = aUV;
    TexIndex = iTexID;
    Index = iAtlas;
    // В UI обычно удобнее считать от верхнего левого угла
    vec2 worldPos = aPos.xy * iScale + iOffset;
    gl_Position = projection * vec4(worldPos, 0.0, 1.0);
}
"#;
pub const FSDDC_SRC: &str = r#"
#version 460 core
out vec4 FragColor;
layout (location = 8)uniform sampler2DArray u_WorldTextures; // Блоки
layout (location = 13)uniform sampler2DArray u_UITextures;    // UI
layout (location = 14) uniform sampler2DArray u_FontTextures;  // Index 2 (Новый)
in vec2 TexCoord;  // Приходит 0..1
in float TexIndex; // Индекс слоя в массиве (0 - хотбар, 1 - селектор и т.д.)
in float Index;
// Привязываем UI массив к 8-му слоту
void main(){
vec4 texColor;
if (Index == 0.0) {
    texColor = texture(u_WorldTextures, vec3(TexCoord.x, 1.0 - TexCoord.y, TexIndex));
} else if (Index == 1.0) {
    texColor = texture(u_UITextures, vec3(TexCoord.x, 1.0 - TexCoord.y, TexIndex));
} else if (Index == 2.0) {
    texColor = texture(u_FontTextures, vec3(TexCoord.x, 1.0 - TexCoord.y, TexIndex));
}

if(texColor.a < 0.25) discard;
FragColor = texColor;
}
"#;
// Коэффициент затухания: 0 -> 0.4 (темно), 3 -> 1.0 (светло)
//float ao_mult = 0.4 + (float(ao_raw) / 3.0) * 0.6;
//vBrightness *= ao_mult;

// #version 460 core
// in vec3 vNormal;
// in float vBrightness;
// in vec2 uv;
// in float vAO;
// in float vLayer;
// in float vLight;

// uniform sampler2DArray mySampler;
// out vec4 FragColor;

// void main() {
//     vec4 texColor = texture(mySampler, vec3(1-uv, vLayer));
//     if(texColor.a < 0.1) discard; // Прозрачность

//     // 1. Нелинейное освещение (Gamma-подобное)
//     // Делает градиент в пещерах более мягким и глубоким
//     float lightFactor = pow(vLight, 1.2);

//     // 2. Минимальный эмбиент (чтобы в полной темноте было хоть что-то видно)
//     float ambient = 0.9;
//     float totalLight = max(lightFactor, ambient);
//         vec3 baseRGB;
//         if (abs(vLayer - 1.0) < 0.1) { // Безопасное сравнение float
//             baseRGB = texColor.ggg * vec3(0.4, 0.8, 0.3);
//         } else {
//             // Вместо сырого rgb, возьмем яркость, чтобы убрать цветовой шум
//             // 0.33 - это усреднение каналов
//             //float luminance = dot(texColor.rgb, vec3(0.33, 0.33, 0.33));
//             //baseRGB = vec3(luminance) * texColor.rgb; // Сохранит родной цвет, но "причешет" его
//             baseRGB = texColor.rgb;
//             // Или просто оставь baseRGB = texColor.rgb, если уверен в flat
//         }
//     // 3. Итоговый расчет
//     // vBrightness - тени сторон, vAO - затенение углов, totalLight - ваш Flood Fill
//     vec3 finalColor = baseRGB * vBrightness * vAO * totalLight;

//     // Добавляем небольшое усиление для насыщенности в темноте
//     finalColor *= 1.1;

//     FragColor = vec4(finalColor, texColor.a);
// }
// #version 460 core
// layout (location = 0) in uint packedData;

// uniform mat4 model;
// uniform mat4 view;
// uniform mat4 projection;

// out float vBrightness;
// out vec3 vNormal;
// out float vAO;
// out vec2 uv;
// out float vLayer;
// out float vLight; // Передаем чистый коэффициент света

// const vec2 corners[4] = vec2[](vec2(0,0), vec2(1,0), vec2(1,1), vec2(0,1));
// const vec3 normals[6] = vec3[](
//     vec3(0,1,0), vec3(0,-1,0), vec3(0,0,1), vec3(0,0,-1), vec3(-1,0,0), vec3(1,0,0)
// );

// void main() {
//     // Распаковка (ваша схема)
//     uint x          =  packedData        & 31u;
//     uint y          = (packedData >> 5u)  & 31u;
//     uint z          = (packedData >> 10u) & 31u;
//     uint corner_id  = (packedData >> 15u) & 3u;
//     uint face_id    = (packedData >> 17u) & 7u;
//     uint t_id       = (packedData >> 20u) & 63u;
//     uint l_id       = (packedData >> 26u) & 15u;
//     uint ao_raw     = (packedData >> 30u) & 3u;

//     vLayer = float(t_id);
//     vNormal = normals[face_id];
//     uv = corners[corner_id];

//     // Перевод света: 15 -> 1.0, 0 -> 0.0
//     vLight = float(l_id) / 15.0;

//     // AO: 3 -> 1.0 (светло), 0 -> 0.4 (темно)
//     vAO = 0.4 + (float(ao_raw) / 3.0) * 0.6;

//     // Константы яркости граней (Fake Global Illumination)
//     float faceShadows[6] = float[](1.0, 0.4, 0.8, 0.6, 0.7, 0.5);
//     vBrightness = faceShadows[face_id];

//     // Позиционирование вершины
//     vec3 pos = vec3(x, y, z);
//     if (face_id == 0u) pos += vec3(uv.x, 1.0, uv.y);
//     else if (face_id == 1u) pos += vec3(uv.x, 0.0, uv.y);
//     else if (face_id == 2u) pos += vec3(uv.x, uv.y, 1.0);
//     else if (face_id == 3u) pos += vec3(uv.x, uv.y, 0.0);
//     else if (face_id == 4u) pos += vec3(1.0, uv.y, uv.x);
//     else if (face_id == 5u) pos += vec3(0.0, uv.y, uv.x);

//     gl_Position = projection * view * model * vec4(pos, 1.0);
// }
