// Firebase v9+ uses a new modular syntax
import { initializeApp } from "firebase/app";
import { getDatabase } from "firebase/database";
import { getFirestore } from "firebase/firestore";

export default defineNuxtPlugin(() => {
  const firebaseConfig = {
    apiKey: "AIzaSyDot-0fDndkYXa9N3QJwvrzwWQxcN3lwdg",
    authDomain: "sportkit-167c0.firebaseapp.com",
    databaseURL:
      "https://sportkit-167c0-default-rtdb.asia-southeast1.firebasedatabase.app",
    projectId: "sportkit-167c0",
    storageBucket: "sportkit-167c0.appspot.com",
    messagingSenderId: "71493237069",
    appId: "1:71493237069:web:97e3343f1cb10c7f929f04",
  };

  // Initialize Firebase
  const app = initializeApp(firebaseConfig);

  // Initialize Firestore
  const firestore = getFirestore(app);

  // Provide Firestore to the app
  return {
    provide: {
      firestore,
    },
  };
});
