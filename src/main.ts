import { invoke } from "@tauri-apps/api/tauri";

/**
 * A mirror class to the backend for ease of use
 *
 * @default has a default implementation, accessed by `let x = new Person()`
 *
 * @params username, age, timestamp, comment
 */
class Person {
  username: string = "John Doe";
  age: number = 34;
  timestamp: string = new Date().toLocaleString();
  comment: string = "amen";
  constructor(
    username: string = "John Doe",
    age: number = 34,
    timestamp: string = new Date().toLocaleString(),
    comment: string = "amen",
  ) {
    this.username = username;
    this.age = age;
    this.timestamp = timestamp;
    this.comment = comment;
  }
  /**Converts a Person instance to a tuple of (Username, Age, Timestamp, Comment)
   *
   *
   * @remarks
   * Basically useless for now
   *
   * @param this: takes in a Person instance
   */
  return_person_tuple(
    this: Person,
  ): [string, number, string, string] {
    let { username, age, timestamp, comment } = this;
    return [username, age, timestamp, comment];
  }
}
// ----------------------------------------------------------------------------------
// setting up event func
/**
 * Invokes necessary backend code for adding a person to the logger
 *
 * @remarks
 * This should be used as a last function (almost always)
 *
 * @param person_array: array of objects of class Person
 * @returns nothing (for now)
 */
const send_person_data = async function (
  person_array: Person[],
  display_elem: HTMLElement | null,
) {
  // for each tuple instance call the backend
  let i = 0;
  for (const person of person_array) {
    i++;
    console.log(i);
    await invoke("accept_person_data", {
      username: person.username,
      age: person.age,
      timestamp: person.timestamp,
      comment: person.comment,
    })
      .then((res) => {
// @ts-ignore
        display_elem.innerText = res;
      }, (e) => {
        display_elem.innerText = e;
      });
  }
};
/**
 * Invokes necessary backend code for flushing the logger to disk
 *
 * the logger object is then reset to a default state
 * @remarks
 * When a PersonLogger is added to the FrontEnd: FIXME: reset the frontend logger as well
 *  
 * @param display_elem: is there to display the answer of the backend
 * @returns nothing (no promises?!)
 */
const flush_logger = async function (display_elem: HTMLElement | null) {
  await invoke("flush_logger")
    .then((res) => {
// @ts-ignore
      display_elem.innerText = res;
    }, (e) => {
      display_elem.innerText = e;
    });
};
window.addEventListener("DOMContentLoaded", () => {
  // setting up vars
  let form_elem: HTMLElement | null;
  let header_elem: HTMLElement | null;

  form_elem = document.querySelector("#form_elem");
  header_elem = document.querySelector("#header");
  // event listener on main form
  form_elem?.addEventListener("submit", (event: Event) => {
    // prevent the default reload action, build the Person from form selectors
    event.preventDefault();

    const person: Person = new Person(
      form_elem[0].value,
      parseInt(form_elem[1].value),
      new Date().toLocaleString(),
      form_elem[2].value,
    );
    // checking what button was pressed
// @ts-ignore
    if (event.submitter.id === "add_person") {
      // send person to backend
      send_person_data([person], header_elem);
    } else {
      // flush the logger to disk
      flush_logger(header_elem);
    }
  });
});
