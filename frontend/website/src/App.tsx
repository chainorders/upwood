import { StrictMode, useState } from 'react';
import { BrowserRouter as Router, Routes, Route, useNavigate } from 'react-router-dom';
import { NextUIProvider } from '@nextui-org/react';
import { MotionConfig } from 'framer-motion';
import { InvitationModal } from './components/InvitationModal';
import { LaunchAppModal } from './components/LaunchAppModal';
import { FloatingCircles } from './components/FloatingCircles';
import { Footer } from './components/Footer';
import { HomePage } from './pages/Home';
import About from './pages/About';
import Learn from './pages/Learn';
import BlogPost from './pages/BlogPost';
import Privacy from './pages/Privacy';
import Contact from './pages/Contact';
import TermsOfUse from './pages/TermsOfUse';
import CookiePolicy from './pages/CookiePolicy';

function App() {
  const [isInvitationModalOpen, setIsInvitationModalOpen] = useState(false);
  const [isLaunchAppModalOpen, setIsLaunchAppModalOpen] = useState(false);

  const handleRequestInvitation = () => {
    // Redirect to platform registration
    window.location.href = 'https://app.greenbond.io/registration';
  };

  return (
    <Router>
      <NextUIProvider>
        <MotionConfig reducedMotion="user">
          <Routes>
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
            <Route path="/about" element={<About />} />
            <Route path="/learn" element={<Learn />} />
            <Route path="/blog/:slug" element={<BlogPost />} />
            <Route path="/privacy" element={<Privacy />} />
            <Route path="/terms" element={<TermsOfUse />} />
            <Route path="/cookies" element={<CookiePolicy />} />
            <Route path="/contact" element={<Contact />} />
          </Routes>

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