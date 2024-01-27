import { invoke } from "@tauri-apps/api";

export default () => {
  if (process.client) {
    window.onbeforeunload = function () {
      // Your logic here
      // For example, sending a message to the Tauri backend
      //   invoke("close_all_processes");
      // Optional: Return a string if you want a confirmation dialog
      // return 'Are you sure you want to leave?';
    };
  }
};
