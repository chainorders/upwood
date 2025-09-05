import React from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { Footer } from '../components/Footer';

export default function TermsOfUse() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      <div className="max-w-4xl mx-auto px-4 py-20">
        <h1 className="text-4xl font-bold text-eco-green mb-8">Website Terms of Use</h1>
        
        <div className="space-y-8 text-gray-300">
          <p>
            These Terms of Use govern the access and use of the Upwood website <a href="http://www.upwood.io" className="text-eco-green hover:underline">www.upwood.io</a> (the "Website"). Use of the words "Upwood", "we", "us", or "our") refers to SIA "Upwood" a private limited liability company registered in Latvia under Reg. nr. 40203494875 with registered address at Riga, Salnas iela 21 – 4.10.
          </p>

          <p>
            By accessing this Website, the pages contained on it, and the information and material contained or described herein (together the "Information"), you acknowledge your agreement with, and understanding of the following terms and conditions of use (the "Terms of Use").
          </p>

          <p>
            In the event of any inconsistency between these Terms of Use and any other papers, policies, terms, conditions, licenses, limitations, or obligations contained within or on the Website, these Terms of Use shall prevail. These Terms of Use should be read in conjunction with:
          </p>

          <ol className="list-decimal list-inside space-y-2 ml-4">
            <li>
              the Privacy Policy <a href="https://www.upwood.io/privacy" className="text-eco-green hover:underline">https://www.upwood.io/privacy</a>; and
            </li>
            <li>
              the Cookies Policy <a href="https://upwood.io/cookies-policy" className="text-eco-green hover:underline">https://upwood.io/cookies-policy</a>
            </li>
          </ol>

          <p>
            To contact us, please email <a href="mailto:info@upwood.io" className="text-eco-green hover:underline">info@upwood.io</a>.
          </p>

          <p>
            These terms may be changed, supplemented or updated by us in our sole discretion at any time without advance notice. We recommend visiting the Terms of Use from time to time. Your continued use of this Website will confirm your acceptance of these terms as modified, changed, supplemented or updated by us. If you do not agree to such revised terms you should stop using this Website and all information, links or content contained on this Website.
          </p>

          <div className="mt-8 space-y-8">
            <div>
              <h2 className="text-2xl font-bold text-white mb-4">1. Use of the Website</h2>
              <div className="space-y-4">
                <p>
                  You shall not use the Website in any way that is fraudulent or unlawful. The Information is not intended for distribution to, or use by, any person or entity in any jurisdiction or country where such distribution or use would be contrary to law or regulation.
                </p>

                <p>
                  The products and services described on the Website are only intended for users who are legally permitted to access and use them. The products and services may not be eligible or available for sale in all jurisdictions or to certain categories of investors. The products and services are not intended for persons subject to a jurisdiction that prohibits the publication of and the access to the Website (due to the nationality of the respective person or on any other grounds). Persons subject to such restrictions are prohibited from accessing the Website.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">2. Purpose of Information</h2>
              <div className="space-y-4">
                <p>
                  The Information on the Website is for information purposes only and is not offered as advice on any particular matter and must not be treated as a substitute for specific advice. In particular, the Information does not constitute professional, financial or investment advice and must not be used as a basis for making investment decisions and is in no way intended, directly or indirectly, as an attempt to market or sell any type of financial instrument. Advice from a suitably qualified professional should always be sought in relation to any particular matter or circumstances.
                </p>

                <p>
                  The Information is not intended to and does not constitute an offer to sell or a solicitation of an offer to buy any securities in any jurisdiction in which such offer or solicitation would be unlawful.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">3. Availability</h2>
              <div className="space-y-4">
                <p>
                  We do not guarantee that our Website, or any of the Information, will always be available or be uninterrupted. We reserve the right to suspend, withdraw, or restrict access to all or any part of the Website at any time, at our sole discretion, and without the need to provide any reason.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">4. Limitation of Liability</h2>
              <div className="space-y-4">
                <p>
                  Upwood does not provide any representation or warranty as to the accuracy, reliability, timeliness or completeness of the Information, nor as to the appropriateness of the Information for any use which any individual user may choose to make of it and accepts no responsibility for updating any part of the Information.
                </p>

                <p>
                  Upwood excludes all implied conditions, warranties, representations or other terms that may apply to the Website or any of the Information. Upwood will not be liable to you for any error, omission or misrepresentation in relation to the Information or for any loss, damage, cost or expense (whether direct, indirect, consequential or otherwise), arising under or in connection with: use of, inability to use, the Website; or use of or reliance on any of the Information.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">5. Intellectual Property</h2>
              <div className="space-y-4">
                <p>
                  The Website including all Information is owned by or licensed to Upwood and is subject to and protected by various intellectual property rights including but not limited to copyright, trade secrets, trademarks, service marks, brand names and other proprietary rights whether under contract, statute or any similar provisions ("IP Rights"). All IP Rights are and shall remain the exclusive property of Upwood and nothing in these Terms of Use gives you any right, title, or ownership of the IP Rights. By accessing and using the Website and the Information, you are entitled to view the Information on the Website and to copy and print such information for personal use. You are not permitted to sell or distribute or otherwise deal with the Information on the Website or any derivations of such information without the prior written consent of Upwood.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">6. Representations and Warranties</h2>
              <div className="space-y-4">
                <p>
                  You represent and warrant to Upwood that, if you are an individual user, you have the capacity to enter into a contract under applicable laws. If you use the Website on behalf of a legal entity, you represent and warrant that, you are duly authorised to act on behalf of such entity.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">7. Personal Information</h2>
              <div className="space-y-4">
                <p>
                  We process personal information in accordance with our Privacy Policy: <a href="https://www.upwood.io/privacy" className="text-eco-green hover:underline">https://www.upwood.io/privacy</a>. By using the Website, you consent to such processing and you warrant that all data provided by you is accurate.
                </p>
              </div>
            </div>

            <div>
              <h2 className="text-2xl font-bold text-white mb-4">8. Governing Law</h2>
              <div className="space-y-4">
                <p>
                  Any dispute, claim or action related to the use of the Website or these Terms of Use shall be governed by the laws of the Republic of Latvia. You hereby consent to the exclusive jurisdiction of the courts of Riga, Latvia.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Footer />
    </div>
  );
}