import type { TodoItem, TodoList, UrlPattern } from "../global";

export const urlPattern = (): UrlPattern => {
  const pattern =
    /(http|ftp|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-])/gm;
  const regex = new RegExp(pattern);
  console.log("calling urlPattern");
  return {
    regex: regex,
    pattern: pattern,
  };
};

const todo = {
  url: "http://localhost:3000",
  fetchList: async function () {
    const response = await fetch(this.url + "/todolist");
    const result: TodoList[] = await response.json();
    return result.reverse();
  },
  deleteList: async function () {
    const response = await fetch(this.url + "/delete");
    return await response.json();
  },
  todoIsDone: async function (doneid: string) {
    const url = this.url + "/done/" + doneid;
    console.log(url);

    const response = await fetch(url);
    return await response.json();
  },
  todoAdd: async function (text: string) {
    const url = this.url + `/add`;
    const datas = text.split(/\[.+].*?:\s/gm);
    for await (const text of datas) {
      if (text.length > 0) {
        await fetch(url, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            sender: "todo@apps",
            text: text,
            done: false,
            checked: false,
            time: new Date(),
          }),
        });
      }
    }
  },
  todoIsCheck: async function (id: string, is_check: boolean) {
    const url = this.url + `/check/${id}/${is_check}`;
    const response = await fetch(url);
    return await response.json();
  },

  hasUrl: function (str: string, pattern: UrlPattern): TodoItem {
    const has_url = str.match(pattern.regex);

    if (has_url) {
      const text = str.replace(pattern.pattern, "");
      const links: string[] = [];
      has_url.forEach((link) => {
        links.push(link);
      });
      return {
        text: text,
        links: links,
      };
    } else {
      return {
        text: str,
        links: [],
      };
    }
  },
};
export default todo;
