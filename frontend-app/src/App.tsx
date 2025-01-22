import { BrowserRouter, Route, Routes } from 'react-router'
import AuthLayout from './AuthLayout.tsx'
import ActiveForestProjectsList from './pages/ActiveForestProjectsList.tsx'
import ActiveForestProjectDetails from './pages/ActiveForestProjectDetails.tsx';
import InvestmentPortfolio from './pages/InvestmentPortfolio.tsx';
import AuthLogin from './pages/AuthLogin.tsx';
import { ApiUser } from './apiClient/index.ts';
import { useState } from 'react';
import Wallet from './pages/Wallet.tsx';
import News from './pages/News.tsx';
import NewsDetails from './pages/NewsDetails.tsx';

export default function App() {
    const [user, setUser] = useState<ApiUser>();

    return <BrowserRouter>
        <Routes>
            <Route path="/login" element={<AuthLogin setUser={setUser} />} />
            <Route path="/" element={<AuthLayout user={user} logout={() => {
                console.log('logout');
                setUser(undefined);
            }} />}>
                <Route index path='projects/active' element={<ActiveForestProjectsList />} />
                <Route index path='projects/active/:id' element={<ActiveForestProjectDetails />} />
                <Route index path='portfolio' element={<InvestmentPortfolio />} />
                <Route index path='wallet' element={<Wallet />} />
                <Route index path='news' element={<News />} />
                <Route index path='news/:id' element={<NewsDetails />} />
            </Route>
        </Routes>
    </BrowserRouter>
}