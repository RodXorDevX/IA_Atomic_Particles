import React from 'react';
import { AtomicCanvas } from './components/AtomicCanvas';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>Simulación de Núcleos Atómicos</h1>
      </header>
      <main>
        <AtomicCanvas width={1200} height={800} />
      </main>
    </div>
  );
}//test2jhjh

export default App; 