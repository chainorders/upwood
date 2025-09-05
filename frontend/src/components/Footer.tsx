import React from 'react';
import { Link } from 'react-router-dom';
import { Linkedin, GitBranch as BrandTelegram, XIcon } from 'lucide-react';

export function Footer() {
  const navigation = {
    main: {
      column1: [
        { name: 'Home', href: '/' },
        { name: 'About Us', href: '/about' },
        { name: 'Learn', href: '/learn' },
        { name: 'Contact', href: '/contact' }
      ],
      column2: [
        { name: 'Privacy Policy', href: '/privacy' },
        { name: 'Terms of Use', href: '/terms' },
        { name: 'Cookie Policy', href: '/cookies' }
      ]
    },
    social: [
      {
        name: 'LinkedIn',
        href: 'https://www.linkedin.com/company/upwood-io/',
        icon: Linkedin,
      },
      {
        name: 'Telegram',
        href: 'https://t.me/+S1rip4Mj9Xg4M2Y0',
        icon: BrandTelegram,
      },
      {
        name: 'X',
        href: 'https://x.com/upwood_io',
        icon: XIcon,
      },
    ],
  };

  return (
    <footer className="bg-black border-t border-gray-800">
      <div className="max-w-7xl mx-auto px-4 py-12 sm:px-6 lg:px-8">
        <div className="xl:grid xl:grid-cols-3 xl:gap-8">
          {/* Company Info - Hidden on mobile, shown on desktop */}
          <div className="hidden xl:block xl:col-span-1">
            <div className="flex items-center gap-2">
              <img 
                src="https://www.upwood.io/images/upwood.png" 
                alt="Upwood Logo" 
                className="h-8"
              />
            </div>
            <div className="text-gray-400 space-y-1 text-sm mt-8">
              <p>SIA "Upwood"</p>
              <p>Reg. No. 40203494875</p>
              <p>
                <a href="mailto:info@upwood.io" className="hover:text-eco-green">
                  info@upwood.io
                </a>
              </p>
              <p>Office: Raņķa Dambis 34,</p>
              <p>Riga, Latvia,</p>
              <p>LV-1048</p>
            </div>
          </div>

          {/* Navigation */}
          <div className="mt-12 xl:mt-0 xl:col-span-2">
            <div className="grid grid-cols-3 gap-8">
              {/* Navigation Column 1 */}
              <div>
                <h3 className="text-sm font-semibold text-eco-green tracking-wider uppercase">
                  Navigation
                </h3>
                <ul className="mt-4 space-y-4">
                  {navigation.main.column1.map((item) => (
                    <li key={item.name}>
                      <Link
                        to={item.href}
                        className="text-base text-gray-400 hover:text-eco-green"
                      >
                        {item.name}
                      </Link>
                    </li>
                  ))}
                </ul>
              </div>

              {/* Navigation Column 2 */}
              <div>
                <h3 className="text-sm font-semibold text-eco-green tracking-wider uppercase">
                  Legal
                </h3>
                <ul className="mt-4 space-y-4">
                  {navigation.main.column2.map((item) => (
                    <li key={item.name}>
                      <Link
                        to={item.href}
                        className="text-base text-gray-400 hover:text-eco-green"
                      >
                        {item.name}
                      </Link>
                    </li>
                  ))}
                </ul>
              </div>

              {/* Social Links */}
              <div>
                <h3 className="text-sm font-semibold text-eco-green tracking-wider uppercase">
                  Follow Us
                </h3>
                <div className="mt-4 space-y-4">
                  {navigation.social.map((item) => (
                    <a
                      key={item.name}
                      href={item.href}
                      className="flex items-center text-gray-400 hover:text-eco-green"
                      target="_blank"
                      rel="noopener noreferrer"
                    >
                      <item.icon className="h-6 w-6 mr-2" />
                      <span>{item.name}</span>
                    </a>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Company Info for Mobile - Hidden on desktop */}
        <div className="xl:hidden mt-12 pt-8 border-t border-gray-800">
          <div className="flex items-center gap-2 mb-6">
            <img 
              src="https://www.upwood.io/images/upwood.png" 
              alt="Upwood Logo" 
              className="h-8"
            />
          </div>
          <div className="text-gray-400 space-y-1 text-sm">
            <p>SIA "Upwood"</p>
            <p>Reg. No. 40203494875</p>
            <p>
              <a href="mailto:info@upwood.io" className="hover:text-eco-green">
                info@upwood.io
              </a>
            </p>
            <p>Office: Raņķa Dambis 34,</p>
            <p>Riga, Latvia,</p>
            <p>LV-1048</p>
          </div>
        </div>

        {/* Copyright Section */}
        <div className="mt-12 border-t border-gray-800 pt-8">
          <p className="text-base text-gray-400 text-center">
            &copy; {new Date().getFullYear()} Upwood. All rights reserved.
          </p>
          <p className="mt-4 text-sm text-gray-500 text-center max-w-4xl mx-auto">
            The content of this webpage is not an investment advice and does not constitute any offer or solicitation to offer or recommendation of any investment product. Investment involves risk is not indicative of future performance. Investors should refer to the offering documentation of the projects for detailed information (including risk factors) prior to investing.
          </p>
        </div>
      </div>
    </footer>
  );
}