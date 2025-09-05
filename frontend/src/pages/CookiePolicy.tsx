import React from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { Footer } from '../components/Footer';

export default function CookiePolicy() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      <div className="max-w-4xl mx-auto px-4 py-20">
        <h1 className="text-4xl font-bold text-eco-green mb-8">Cookie Policy</h1>
        
        <div className="space-y-8 text-gray-300">
          <section>
            <p className="leading-relaxed">
              This Cookie Policy provides you with information about the cookies we use and the purposes for which we use them. We use cookies to ensure that Upwood website <a href="https://www.upwood.io" className="text-eco-green hover:underline">https://www.upwood.io</a> (the "Site") functions to the highest standard. Use of the words "Upwood" ("we", "us," or "our") refers to SIA "Upwood" a private limited liability company registered in Latvia under Reg. nr. 40203494875 with registered address at Riga, Salnas iela 21 - 410.
            </p>
            <p className="mt-4">
              Cookies are necessary and some of them are set automatically. We would also like to use some cookies to make your visit more personal, improve our Site based on how you use it and support our marketing. These cookies are optional and you can choose which types you would like to accept. You can manage your cookie preferences by using your browser.
            </p>
            <p className="mt-4">
              By visiting and continuing to use our Site, you are consenting to our use of cookies and related technologies to provide our services. If you do not consent to our cookies, you may stop using our services and stop visiting our Site. You can disable cookies in your browser, but doing so may interfere with your use of our Site and services.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">1. What are cookies?</h2>
            <p className="leading-relaxed">
              Cookies are small pieces of data that websites store on your browser when you visit them. Cookies are useful because they allow a website to recognize your visit and collect information about how you use the website.
            </p>
            <p className="mt-4">
              The length of time for which cookies are stored on your browser varies depending on the cookie. Some cookies only last for your online session, whereas others will stay on your device for a longer period. We aim to set our cookies for no longer than 12 months and are working with our trusted partners to ensure cookies are always set for a reasonable period of time.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">2. The cookies we use</h2>
            <div className="space-y-6">
              <div>
                <h3 className="text-xl font-semibold text-white mb-2">Strictly necessary cookies</h3>
                <p className="text-gray-400 mb-4">
                  These cookies are necessary for the operation of our Site. We don't have to ask for your consent to store these cookies on your browser. We listed the categories of these strictly necessary cookies below.
                </p>

                <div className="space-y-4 pl-4">
                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that help you log on</h4>
                    <p className="text-gray-400">
                      We use these cookies to remember who you are when you log on to secure areas of our Site. You won't be able to log on without them.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that help us provide core services and features</h4>
                    <p className="text-gray-400">
                      We use these cookies to provide core services and features on our Site. These services and features won't work without them.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that help keep our Site secure</h4>
                    <p className="text-gray-400">
                      We use these cookies to protect the security of our Site, for example, to make sure the Site is only accessed by genuine users. This helps us keep you safe.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that help us detect fraud or crime</h4>
                    <p className="text-gray-400">
                      We use these cookies to help us identify suspicious behaviour on our Site so we can protect both you and us from fraud.
                    </p>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="text-xl font-semibold text-white mb-2">Optional cookies</h3>
                <p className="text-gray-400 mb-4">
                  These cookies are optional and you can choose whether to accept them or not. You can manage your cookie preferences through your browser settings.
                </p>

                <div className="space-y-4 pl-4">
                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that we use to make your visit more personal</h4>
                    <p className="text-gray-400">
                      We use these cookies to ensure our optional features and services work. Our Site will still work without them. Cookies that allow us to customise what you see on our Site and where, based on what we know about you. Cookies that help prevent fraud on other websites or services that you haven't asked to use when you're on our Site. Cookies that help keep secure other websites or services that you haven't asked to use when you're on our website.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that help us improve our Site</h4>
                    <p className="text-gray-400">
                      We use these cookies to help us understand how people use our Site. We can then use this data to improve how our Site works. For instance, we may use analytics providers to identify and count visits to our Site and to see which pages people go to and what they do there.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-1">Cookies that support marketing</h4>
                    <p className="text-gray-400">
                      We and our partners use these cookies to understand what you're interested in on our Site and on social media. These cookies may also identify which other websites may have referred you to our Site. This is so we or our partners can personalise our marketing to you, including online advertising and through email, telephone, text, secure message or social media.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">3. Managing cookies</h2>
            <div className="space-y-4">
              <p className="leading-relaxed">
                You can change your mind on how you receive certain types of marketing message or choose to stop receiving them at any time.
              </p>
              
              <p className="leading-relaxed">
                We understand that not everyone likes data to be collected about them when it is not strictly necessary. You can manage your cookie preferences by using your browser.
              </p>

              <p className="leading-relaxed">
                Your preferences are stored in cookies stored on your browser. If you switch off a category of cookies that you previously accepted, then for technical reasons those cookies will not be deleted. To delete cookies from your browser, we recommend that you clear your browser data. If you do this, or change your browser settings, we will ask for your preferences again when you next visit our website.
              </p>

              <p className="leading-relaxed">
                You can use your browser settings to delete cookies that have already been set at any time. You can also use your browser settings to manage cookies, for example, to switch off a cookie altogether. If you do this, it could mean that we can't use strictly necessary cookies properly and some parts of our website may not work correctly.
              </p>

              <p className="leading-relaxed">
                For more information about how to use your browser settings to clear your browser data or to manage cookies, check your browser 'Help' function. We are not responsible for the functionality, content and services provided by the service provider of your browser.
              </p>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">4. Cookies and your privacy</h2>
            <div className="space-y-4">
              <p className="leading-relaxed">
                The information cookies collect, and how we use that information, may count as Personal Data. We may also be able to identify you by name, IP address or session ID. You have rights regarding how we collect, store and use your Personal Data. You can learn more about how we use your Personal Data in our Privacy Policy. If you have any questions about this Cookie Policy, please contact us by emailing dataprotection@upwood.io.
              </p>
            </div>
          </section>
        </div>
      </div>

      <Footer />
    </div>
  );
}