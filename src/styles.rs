use stylist::{css, Style};

pub fn responsive_styles() -> Style {
  Style::new(css!(
      r#"
        .container {
            width: 95%; /* 画面幅に合わせてコンテナの幅を調整 */
            max-width: 1200px; /* 最大幅を設定 */
            margin: 0 auto; /* 中央寄せ */
            height: auto; /* 高さを自動調整 */
        }

        @media (min-width: 768px) {
            .container {
                width: 70%;
            }
        }

        @media (min-width: 1200px) {
            .container {
                width: 50%;
            }
        }
        "#
  ))
  .unwrap()
}

pub fn app_styles() -> Style {
  Style::new(css!(
  r#".logo.yew:hover {
  filter: drop-shadow(0 0 2em #20a88a);
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
"#
  ))
  .unwrap()
}

pub fn overlay_style() -> Style {
  Style::new(css!(
  r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.8); /* 透明度を少し暗めに */
        display: flex;
        flex-direction: column;
        justify-content: center; /* 中央配置 */
        align-items: center; /* 中央配置 */
        z-index: 200; /* nav より前面に */
        opacity: 0;
        visibility: hidden;
        transition: opacity 0.3s ease, visibility 0.3s ease;

        &.is-opened {
            opacity: 1;
            visibility: visible;
        }
  "#
))
.unwrap()
}

pub fn menu_style() -> Style {
  Style::new(css!(
  r#"
        background: white;
        padding: 40px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        row-gap: 20px;
        align-items: center;
        width: 80%;
        max-width: 400px;
        text-align: center;
        z-index: 201;

  "#
))
.unwrap()
}


pub fn menu_list_style() -> Style {
  Style::new(css!(
      r#"
      list-style: none;
      padding: 0;
      margin: 0;
      display: flex;
      flex-direction: column;
      align-items: center;
      row-gap: 15px;
      font-size: 20px;
      font-weight: bold;
      color: #333;
      "#
  ))
  .unwrap()
}

pub fn menu_button_style() -> Style {
  Style::new(css!(
      r#"
      height: 45px;
      width: 45px;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      row-gap: 6px;

      &__line,
      &::before,
      &::after {
          content: "";
          width: 28px;
          height: 2px;
          background-color: #333333;
          transition: transform 0.3s, opacity 0.3s;
      }

      &.is-opened &__line {
          opacity: 0;
      }

      &.is-opened::before {
          transform: translateY(8px) rotate(45deg);
      }

      &.is-opened::after {
          transform: translateY(-8px) rotate(-45deg);
      }
      "#
  ))
  .unwrap()
}

pub fn nav_styles() -> Style {
  Style::new(css!(
      r#"
          position: fixed;
          top: 0;
          left: 0;
          width: 100%;
          height: 100px; /* 画面上端まで拡張 */
          background: #2f2f2f;
          display: flex;
          align-items: center;
          justify-content: space-between;
          padding: 0 20px;
          z-index: 100;
      "#
  )).unwrap()
}

pub fn menu_items() -> Style {
  Style::new(css!(
      r#"
          display: block;
          text-decoration: none;
          color: black;
          margin-right: 35px;
      "#
  )).unwrap()
}

pub fn light_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #0f0f0f;
          background-color: #f6f6f6;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn dark_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #f6f6f6;
          background-color: #2f2f2f;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn toggle_button() -> Style {
  Style::new(css!(
      r#"   
            position: absolute;
            top: 10px;
            right: 20px;
            z-index: 150;
            
            display: flex;
            width: 56px;
            height: 28px;
            border: 1px solid #555555;
            border-radius: 9999px;
            background-color: #dddddd;
            cursor: pointer;

          :has(:focus-visible) {
            outline: auto;
            outline: auto -webkit-focus-ring-color;
          }

      "#
  )).unwrap()
}

pub fn toggle_slider() -> Style {
  Style::new(css!(
      r#"
          
            appearance: none;
            position: absolute;
            top: 10px;
            left: 0px;
            width: 28px;
            height: 28px;
            border: 1px solid #555555;
            border-radius: 9999px;
            transform: translateY(-40%);
            outline: none;
            background-color: #ffffff;
            transition: left 0.2s;
            cursor: pointer;
          

          :checked {
            left: calc(100% - 32px);
            background-color: #2f2f2f;
          }
      "#
  )).unwrap()
}

pub fn title_logo() -> Style {
  Style::new(css!(
      r#"
          width: 500px;
          height: 500px;
          max-width: 100%;
          max-height: 100%;

          @media (max-width: 768px) { /* 768px以下（スマホなど）の場合 width/height は 3/4 にする */
              width: 300px;
              height: 300px;
              max-width: 75%;
              max-height: 100%;
          }
      "#
  )).unwrap()
}

pub fn grid_style() -> Style {
  Style::new(css!(
      r#"
          display: grid;
          grid-template-columns: repeat(30, 50px);
          grid-template-rows: repeat(5, 60px);
          gap: 4px;
          background: #f0f0f0;
          padding: 20px;
          border-radius: 10px;
          box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
      "#
  )).unwrap()
}

pub fn cell_style() -> Style { 
  Style::new(css!(
      r#"
          width: 50px;
          height: 60px;
          background: white;
          border: 1px solid #ddd;
          display: flex;
          justify-content: center;
          align-items: center;
      "#
  )).unwrap()
}

pub fn task_style() -> Style {
  Style::new(css!(
      r#"
          color: white;
          text-align: center;
          padding: 8px;
          cursor: pointer;
          border-radius: 5px;
          box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
          display: flex;
          justify-content: center;
          align-items: center;
      "#
  )).unwrap()
}