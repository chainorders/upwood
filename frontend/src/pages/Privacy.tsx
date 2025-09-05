import React from 'react';
import { Navigation } from '../components/Navigation';
import { useNavigate } from 'react-router-dom';
import { Footer } from '../components/Footer';

export default function Privacy() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen bg-black text-white">
      <Navigation onLaunchApp={() => navigate('/')} />
      
      <div className="max-w-4xl mx-auto px-4 py-20">
        <h1 className="text-4xl font-bold text-eco-green mb-8">Privacy Policy</h1>
        
        <div className="space-y-8 text-gray-300">
          <section>
            <h2 className="text-2xl font-bold text-white mb-4">Background</h2>
            <p className="leading-relaxed">
              SIA Upwood understands that your privacy is important to you and that you care about how your personal data is used. We respect and value the privacy of everyone who visits this website, www.upwood.io ("the Site") and will only collect and use personal data in ways that are described here, and in a way that is consistent with our obligations and your rights under the law.
            </p>
            <p className="mt-4 font-medium">
              Please read this Privacy Policy carefully and ensure that you understand it. By continuing to use the website you accept this Privacy Policy.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">Information About Us</h2>
            <p className="leading-relaxed">
              The Site is owned and operated by SIA Upwood, a Limited Liability Company registered in Latvia under company number 40203494875.
            </p>
            <div className="mt-4 space-y-2">
              <p>Registered address: Salnas iela 21 - 410, Riga, Latvia.</p>
              <p>Email address: <a href="mailto:info@upwood.io" className="text-eco-green hover:underline">info@upwood.io</a></p>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">What Does This Policy Cover?</h2>
            <p className="leading-relaxed">
              This Privacy Policy applies only to your use of the Site. The Site may contain links to other websites. Please note that we have no control over how your data is collected, stored, or used by other websites and we advise you to check the privacy policies of any such websites before providing any data to them.
            </p>
            <p className="mt-4 leading-relaxed">
              Personal data is defined by the EU General Data Protection Regulation ("Data Protection Legislation") as 'any information relating to an identifiable person who can be directly or indirectly identified in particular by reference to an identifier'.
            </p>
            <p className="mt-4 leading-relaxed">
              Personal data is, in simpler terms, any information about you that enables you to be identified. Personal data covers obvious information such as your name and contact details, but it also covers less obvious information such as identification numbers, electronic location data, and other online identifiers.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">What Are My Rights?</h2>
            <p className="mb-4">Under the Data Protection Legislation, you have the following rights, which we will always work to uphold:</p>
            <div className="space-y-4">
              <div>
                <p className="font-medium">a) The right to be informed</p>
                <p className="text-gray-400">About our collection and use of your personal data.</p>
              </div>
              <div>
                <p className="font-medium">b) The right to access</p>
                <p className="text-gray-400">The personal data we hold about you.</p>
              </div>
              <div>
                <p className="font-medium">c) The right to rectification</p>
                <p className="text-gray-400">If any of your personal data is inaccurate or incomplete.</p>
              </div>
              <div>
                <p className="font-medium">d) The right to be forgotten</p>
                <p className="text-gray-400">The right to ask us to delete your personal data.</p>
              </div>
              <div>
                <p className="font-medium">e) The right to restrict processing</p>
                <p className="text-gray-400">The right to limit how we use your personal data.</p>
              </div>
              <div>
                <p className="font-medium">f) The right to object</p>
                <p className="text-gray-400">To our use of your personal data for particular purposes.</p>
              </div>
              <div>
                <p className="font-medium">g) The right to withdraw consent</p>
                <p className="text-gray-400">To withdraw your consent for us to use your personal data.</p>
              </div>
              <div>
                <p className="font-medium">h) The right to data portability</p>
                <p className="text-gray-400">To receive your data in a commonly used format.</p>
              </div>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">What Data Do You Collect and How?</h2>
            <div className="bg-gray-900/50 rounded-lg border border-gray-800 overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-800">
                    <th className="px-6 py-4 text-left">Data Collected</th>
                    <th className="px-6 py-4 text-left">How We Collect the Data</th>
                  </tr>
                </thead>
                <tbody>
                  <tr className="border-b border-gray-800">
                    <td className="px-6 py-4">Contact information including name, surname, email</td>
                    <td className="px-6 py-4">When website user submits contact form</td>
                  </tr>
                  <tr className="border-b border-gray-800">
                    <td className="px-6 py-4">Identity information including name, surname</td>
                    <td className="px-6 py-4">When website user submits contact form</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4">Location data including IP address</td>
                    <td className="px-6 py-4">When website user enters the website and agrees to cookie policy</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">How Do You Use My Personal Data?</h2>
            <div className="bg-gray-900/50 rounded-lg border border-gray-800 overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-800">
                    <th className="px-6 py-4 text-left">What We Do</th>
                    <th className="px-6 py-4 text-left">What Data We Use</th>
                    <th className="px-6 py-4 text-left">Our Lawful Basis</th>
                  </tr>
                </thead>
                <tbody>
                  <tr className="border-b border-gray-800">
                    <td className="px-6 py-4">Registering you on Our site</td>
                    <td className="px-6 py-4">Name, email address</td>
                    <td className="px-6 py-4">Legitimate interest</td>
                  </tr>
                  <tr className="border-b border-gray-800">
                    <td className="px-6 py-4">Communicating with you</td>
                    <td className="px-6 py-4">Contact information</td>
                    <td className="px-6 py-4">Legitimate interest</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4">Supplying you with information by email that you have opted-in-to</td>
                    <td className="px-6 py-4">Email address</td>
                    <td className="px-6 py-4">Consent, legitimate interest</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">How Long Will You Keep My Personal Data?</h2>
            <p className="leading-relaxed">
              We will not keep your personal data for any longer than is necessary in light of the reason(s) for which it was first collected. Your personal data will therefore be kept for the period 5 years for the purposes of legal action.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">How and Where Do You Store or Transfer My Personal Data?</h2>
            <p className="leading-relaxed">
              We will only store your personal data within the European Union (the "EU") with server location being member state Netherlands. The EU consists of all EU member states. This means that your personal data will be fully protected under the EU GDPR and/or to equivalent standards by law.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">Do You Share My Personal Data?</h2>
            <p className="leading-relaxed">
              We will not share any of your personal data with any third parties for any purposes, subject to the following exceptions:
            </p>
            <ul className="list-disc list-inside mt-4 space-y-2">
              <li>If we sell, transfer, or merge parts of our business or assets, your personal data may be transferred to a third party.</li>
              <li>If we are legally required to share certain personal data, which might include yours, if we are involved in legal proceedings or complying with legal obligations, a court order, or the instructions of a government authority.</li>
            </ul>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">How Can I Access My Personal Data?</h2>
            <p className="leading-relaxed">
              If you want to know what personal data we have about you, you can ask us for details of that personal data and for a copy of it (where any such personal data is held). This is known as a "subject access request".
            </p>
            <p className="mt-4 leading-relaxed">
              All subject access requests should be made in writing and sent to the email address shown in Part 12.
            </p>
            <p className="mt-4 leading-relaxed">
              There is not normally any charge for a subject access request. If your request is 'manifestly unfounded or excessive' (for example, if you make repetitive requests) a fee may be charged to cover our administrative costs in responding.
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">How Do I Contact You?</h2>
            <p className="leading-relaxed">
              To contact us about anything to do with your personal data and data protection, including to make a subject access request, please use the following details:
            </p>
            <p className="mt-4">
              Email address: <a href="mailto:info@upwood.io" className="text-eco-green hover:underline">info@upwood.io</a>
            </p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">Changes to this Privacy Policy</h2>
            <p className="leading-relaxed">
              We may change this Privacy Policy from time to time. This may be necessary, for example, if the law changes, or if we change our business in a way that affects personal data protection.
            </p>
            <p className="mt-4 leading-relaxed">
              Any changes will be immediately posted on our Site and you will be deemed to have accepted the terms of the Privacy Policy on your first use of our Site following the alterations. We recommend that you check this page regularly to keep up-to-date.
            </p>
            <p className="mt-4 font-medium">This Privacy Policy was last updated on 16.08.2023.</p>
          </section>

          <section>
            <h2 className="text-2xl font-bold text-white mb-4">Data State Inspectorate</h2>
            <p className="leading-relaxed">
              You can contact Latvian state data inspectorate "Data valsts inspekcija" using following details:
            </p>
            <div className="mt-4 space-y-2">
              <p>Email address: <a href="mailto:pasts@dvi.gov.lv" className="text-eco-green hover:underline">pasts@dvi.gov.lv</a></p>
              <p>Website: <a href="https://www.dvi.gov.lv/" className="text-eco-green hover:underline">https://www.dvi.gov.lv/</a></p>
            </div>
            <p className="mt-4 leading-relaxed">
              If you have any questions or concerns about Privacy Policy, please contact us.
            </p>
          </section>
        </div>
      </div>

      <Footer />
    </div>
  );
}