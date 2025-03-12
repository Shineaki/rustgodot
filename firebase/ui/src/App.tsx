// import { Button } from "@/components/ui/button"
import { LoginForm } from "./components/login-form"
import { initializeApp } from "firebase/app";
import { getAuth } from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyBp75i9u6CQmOo6mVvyLSfbf7ItQ3_GNiM",
  authDomain: "rustgodotgame.firebaseapp.com",
  projectId: "rustgodotgame",
  storageBucket: "rustgodotgame.firebasestorage.app",
  messagingSenderId: "797972530821",
  appId: "1:797972530821:web:1c912ac34ddc9888284bec",
  measurementId: "G-9EM994RY23"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);

// Initialize Firebase Authentication and get a reference to the service
const auth = getAuth(app);

function App() {
  return (
    <div className="flex flex-col items-center justify-center min-h-svh">
      <LoginForm />
    </div>
  )
}

export default App
