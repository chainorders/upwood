import { StrictMode, useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom';
import { NextUIProvider } from '@nextui-org/react';
import { MotionConfig } from 'framer-motion';
import { InvitationModal } from './components/InvitationModal';
import { LaunchAppModal } from './components/LaunchAppModal';
import { FloatingCircles } from './components/FloatingCircles';
import { Footer } from './components/Footer';
import { PlatformLayout } from './components/platform/PlatformLayout';
import { HomePage } from './pages/Home';
import About from './pages/About';
import Learn from './pages/Learn';
import BlogPost from './pages/BlogPost';
import Privacy from './pages/Privacy';
import Contact from './pages/Contact';
import TermsOfUse from './pages/TermsOfUse';
import CookiePolicy from './pages/CookiePolicy';
import Registration from './pages/Registration';
import ActiveProjects from './pages/platform/ActiveProjects';
import MyAssets from './pages/platform/MyAssets';
import Contracts from './pages/platform/Contracts';
import WalletManagement from './pages/platform/WalletManagement';
import ETrees from './pages/platform/ETrees';
import ProfileManagement from './pages/platform/ProfileManagement';
import Settings from './pages/platform/Settings';

function ScrollToTop() {
  const { pathname } = useLocation();

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);

  return null;
}

function App() {
  const [isInvitationModalOpen, setIsInvitationModalOpen] = useState(false);
  const [isLaunchAppModalOpen, setIsLaunchAppModalOpen] = useState(false);

  const handleRequestInvitation = () => {
    setIsInvitationModalOpen(true);
  };

  return (
    <Router>
      <NextUIProvider>
        <MotionConfig reducedMotion="user">
          <ScrollToTop />
          <Routes>
            {/* Public Routes */}
            <Route path="/" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <FloatingCircles />
                <HomePage 
                  onRequestInvitation={handleRequestInvitation}
                  onLaunchApp={() => setIsLaunchAppModalOpen(true)}
                />
                <Footer />
              </div>
            } />
            <Route path="/about" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <FloatingCircles />
                <About />
                <Footer />
              </div>
            } />
            <Route path="/learn" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <FloatingCircles />
                <Learn />
                <Footer />
              </div>
            } />
            <Route path="/blog/:slug" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <BlogPost />
                <Footer />
              </div>
            } />
            <Route path="/privacy" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <Privacy />
                <Footer />
              </div>
            } />
            <Route path="/terms" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <TermsOfUse />
                <Footer />
              </div>
            } />
            <Route path="/cookies" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <CookiePolicy />
                <Footer />
              </div>
            } />
            <Route path="/contact" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <Contact />
                <Footer />
              </div>
            } />
            <Route path="/invite" element={
              <div className="dark min-h-screen bg-black text-white font-sans">
                <Registration />
                <Footer />
              </div>
            } />

            {/* Platform Routes */}
            <Route path="/platform" element={<PlatformLayout />}>
              <Route path="projects" element={<ActiveProjects />} />
              <Route path="assets" element={<MyAssets />} />
              <Route path="contracts" element={<Contracts />} />
              <Route path="wallet" element={<WalletManagement />} />
              <Route path="etrees" element={<ETrees />} />
              <Route path="profile" element={<ProfileManagement />} />
              <Route path="settings" element={<Settings />} />
            </Route>
          </Routes>

          {/* Modals */}
          <InvitationModal 
            isOpen={isInvitationModalOpen}
            onClose={() => setIsInvitationModalOpen(false)}
          />
          <LaunchAppModal 
            isOpen={isLaunchAppModalOpen}
            onClose={() => setIsLaunchAppModalOpen(false)}
            onRequestInvitation={handleRequestInvitation}
          />
        </MotionConfig>
      </NextUIProvider>
    </Router>
  );
}

export default App;