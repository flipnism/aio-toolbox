import {
  exists,
  BaseDirectory,
  readTextFile,
  createDir,
  writeTextFile,
} from "@tauri-apps/api/fs";

function generateConfig(config_file: string) {
  const data = JSON.parse(localStorage.getItem("data"));
  const data_flat = data.map((layer: { id: any; data: any[] }) => {
    return {
      id: layer.id,
      data: layer.data.map((row) => {
        return row.map((keyData) => {
          if (keyData.data_mode == "action") {
            return {
              key: keyData.key,
              data: {
                fromserver: true,
                type: "hotkey",
                data: keyData.data,
              },
              data_mode: keyData.data_mode,
            };
          } else {
            return {
              key: keyData.key,
              data_macro: keyData.data_macro,
              data_mode: keyData.data_mode,
              macro_title: keyData.macro_title,
            };
          }
        });
      }),
    };
  });

  writeTextFile(config_file, JSON.stringify(data_flat, null, 2), {
    dir: BaseDirectory.AppData,
    append: false,
  })
    .then((result) => {
      console.log(result);
    })
    .catch((e) => console.log(e));
}
