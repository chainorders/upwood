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
import ForgotPassword from './pages/ForgotPassword.tsx';
import Register from './pages/Register.tsx';
import Support from './pages/Support.tsx';
import Settings from './pages/Settings.tsx';
import FundedForestProjectsList from './pages/FundedForestProjectsList.tsx';
import FundedForestProjectDetails from './pages/FundedForestProjectDetails.tsx';
import Contracts from './pages/Contracts.tsx';
import ContractsDetails from './pages/ContractsDetails.tsx';

export default function App() {
    const [user, setUser] = useState<ApiUser>();

    return <BrowserRouter>
        <Routes>
            <Route path="/login" element={<AuthLogin setUser={setUser} />} />
            <Route path="/forgot-password" element={<ForgotPassword />} />
            <Route path="/register" element={<Register />} />
            <Route path="/" element={<AuthLayout user={user} logout={() => {
                console.log('logout');
                setUser(undefined);
            }} />}>
                <Route index path='projects/active' element={<ActiveForestProjectsList />} />
                <Route index path='projects/funded' element={<FundedForestProjectsList />} />
                <Route index path='projects/active/:id' element={<ActiveForestProjectDetails />} />
                <Route index path='projects/funded/:id' element={<FundedForestProjectDetails />} />
                <Route index path='portfolio' element={<InvestmentPortfolio />} />
                <Route index path='wallet' element={<Wallet />} />
                <Route index path='news' element={<News />} />
                <Route index path='contracts' element={<Contracts />} />
                <Route index path='contracts/:id' element={<ContractsDetails />} />
                <Route index path='news/:id' element={<NewsDetails />} />
                <Route index path='support' element={<Support />} />
                <Route index path='settings' element={<Settings />} />
            </Route>
        </Routes>
    </BrowserRouter>
}