import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import type { GlobalSetting } from "../global";

export default async function getSettingFor(key: string) {
  const result = await readTextFile("users/settings.json", {
    dir: BaseDirectory.AppData,
  });
  const settings: GlobalSetting[] = JSON.parse(result);
  return settings.filter((setting) => setting.name === key);
}
