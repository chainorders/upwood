import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './styles/index.css';
import './styles/grid.css';
import './styles/globals.css';
import './styles/verticleNavigation.css';
import './styles/buyShare.css';
import './styles/projectCard.css';
import './styles/projectDetail.css';
import './styles/authtextslider.css';
import './styles/support.css';
import './styles/news.css';
import './styles/newsCard.css';
import './styles/newsDetail.css';
import './styles/wallet.css';
import './styles/investmentportfolio.css';
import './styles/contract.css';
import './styles/settings.css';
import './styles/responsive.css';
import App from './App';


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
