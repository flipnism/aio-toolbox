export type Item = {
  id: number;
  value: string;
};
export type GlobalSetting =
  | {
      name: string;
      type: "string";
      value?: string;
      values?: Item[];
    }
  | {
      name: string;
      type: "array";
      value?: string;
      values?: Item[];
    }
  | {
      name: string;
      type: "number";
      value?: number;
      values?: Item[];
    }
  | {
      name: string;
      type: "bool";
      value?: boolean;
      values?: Item[];
    };

export type TodoList = {
  checked: boolean;
  done: boolean;
  sender: string;
  text: string;
  time: string;
  _id: string;
};
export type UrlPattern = {
  regex: RegExp;
  pattern: RegExp;
};

export type TodoItem = {
  text: string;
  links: string[];
};



 export type AppConfig = {
    click_throught: boolean;
    always_on_top: boolean;
    follow_cursor: boolean;
    hold_delay: number;
    reset_delay: number;
    hide_delay: number;
    window_position: Array<[number, number]>;
  };
