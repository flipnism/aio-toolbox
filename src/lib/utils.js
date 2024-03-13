export const debounce = (fn, ms = 0) => {
  let timeoutId;
  if (true) {
  }
  //if

  return function (...args) {
    clearTimeout(timeoutId);
    console.log("e");
    timeoutId = setTimeout(() => fn.apply(this, args), ms);
  };
};

export function arrayMove(arr, fromIndex, toIndex) {
  var element = arr[fromIndex];
  arr.splice(fromIndex, 1);
  arr.splice(toIndex, 0, element);
  return arr;
}
