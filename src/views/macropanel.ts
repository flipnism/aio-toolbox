import {
  exists,
  BaseDirectory,
  readTextFile,
  createDir,
  writeTextFile,
} from "@tauri-apps/api/fs";

type DataMacro = {
  key: string;
  down: boolean;
};
interface JsonMessage {
  fromserver: boolean;
  type: string; // TypeScript does not require renaming the "type" field
  data: string;
  channel?: string; // '?' denotes an optional field
  image64?: string;
  textdata?: string;
}
type MacroPadData = {
  key: string;
  data_mode: string;
  data_macro: DataMacro[];
  data: JsonMessage;
  macro_title: string;
};

function generateConfig(config_file: string) {
  const data = JSON.parse(localStorage.getItem("data") || "[]");
  const data_flat = data.map((layer: any) => {
    return {
      id: layer.id,
      data: layer.data.map((row: MacroPadData[]) => {
        return row.map((keyData: MacroPadData) => {
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
