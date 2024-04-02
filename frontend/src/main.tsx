import React from 'react';
import ReactDOM from 'react-dom/client';

import { BaseApp } from './pages';
import './main.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BaseApp />
  </React.StrictMode>
);
