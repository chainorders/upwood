import { BrowserRouter, Route, Routes } from 'react-router'
import AuthLayout from './AuthLayout.tsx'
import ActiveForestProjectsList from './pages/ActiveForestProjectsList.tsx'
import ActiveForestProjectDetails from './pages/ActiveForestProjectDetails.tsx';
import InvestmentPortfolio from './pages/InvestmentPortfolio.tsx';
import Login from './pages/Login.tsx';
import { ApiUser } from './apiClient/index.ts';
import { useState } from 'react';

export default function App() {
    const [user, setUser] = useState<ApiUser>();

    return <BrowserRouter>
        <Routes>
            <Route path="/login" element={<Login setUser={setUser} />} />
            <Route path="/" element={<AuthLayout user={user} logout={() => {
                console.log('logout');
                setUser(undefined);
            }} />}>
                <Route index path='projects/active' element={<ActiveForestProjectsList />} />
                <Route index path='projects/active/:id' element={<ActiveForestProjectDetails />} />
                <Route index path='portfolio' element={<InvestmentPortfolio />} />
            </Route>
        </Routes>
    </BrowserRouter>
}