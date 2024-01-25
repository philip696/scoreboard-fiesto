export default defineNuxtPlugin(() => {
  if (process.client) {
    window.addEventListener("contextmenu", (event) => {
      event.preventDefault();
    });
  }
});
