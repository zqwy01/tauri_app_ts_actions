import { invoke } from "@tauri-apps/api/tauri";

let form_elem: HTMLElement | null;
let header_elem: HTMLElement | null;
// so the goal here is to send a req to the backend, got it?
const print_text = async function (event: Event) {
  event.preventDefault();
  const date = new Date();
  await invoke("accept_person_data", {
    name: form_elem.querySelector("#username").value,
    age: parseInt(form_elem?.querySelector("#age").value),
    timestamp: date.toLocaleString(),
    comment: form_elem.querySelector("#comment").value,
  })
    .then((res) => {
      header_elem.innerHTML = `<p>${res}</p>`;
    })
    .catch((e) => {
      header_elem.innerHTML = e;
    });
};
window.addEventListener("DOMContentLoaded", () => {
  form_elem = document.querySelector("#form_elem");
  header_elem = document.querySelector("#header");

  form_elem?.addEventListener("submit", print_text);
});
// let greetInputEl: HTMLInputElement | null;
// let greetMsgEl: HTMLElement | null;
//
// async function greet() {
//   if (greetMsgEl && greetInputEl) {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsgEl.textContent = await invoke("greet", {
//       name: greetInputEl.value,
//     });
//   }
// }
//
// greetInputEl = document.querySelector("#greet-input");
// greetMsgEl = document.querySelector("#greet-msg");
// document
//   .querySelector("#greet-button")
//   ?.addEventListener("click", () => greet());
