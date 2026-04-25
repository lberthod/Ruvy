import { initializeApp } from "firebase/app";
import { getDatabase, ref, get, set } from "firebase/database";

const firebaseConfig = {
  apiKey: "AIzaSyANSoSusZ1eMjcZi9gXfWDbGtrM_feHw2s",
  authDomain: "ruvy-da318.firebaseapp.com",
  databaseURL: "https://ruvy-da318-default-rtdb.europe-west1.firebasedatabase.app",
  projectId: "ruvy-da318",
  storageBucket: "ruvy-da318.firebasestorage.app",
  messagingSenderId: "389926682081",
  appId: "1:389926682081:web:db7f94a240bde4ec07e6a0",
  measurementId: "G-83RER47ML8"
};

const app = initializeApp(firebaseConfig);
const database = getDatabase(app);

export { database, ref, get, set };
