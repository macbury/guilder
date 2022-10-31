import React from 'react';
import { createRoot } from 'react-dom/client';

import 'theme.less';
import { BrowserRouter } from "react-router-dom";
import CustomProvider from 'rsuite/CustomProvider';
import Layout from "./components/Layout";
import AuthenticationManager from "./components/authentication";
import Providers from './providers'
import Routes from "./routes";
import CategoryForm from './pages/Categories/CategoryForm';
import StoreProvider from './store';
import AccountForm from './pages/Accounts/AccountForm';

function App() {
  return (
    <BrowserRouter>
      <StoreProvider>
        <AuthenticationManager>
          <CustomProvider theme="dark">
            <Providers>
              <Layout>
                <Routes />
                <CategoryForm />
                <AccountForm />
              </Layout>
            </Providers>
          </CustomProvider>
        </AuthenticationManager>
      </StoreProvider>
    </BrowserRouter>
  )
}

document.addEventListener('DOMContentLoaded', () => {
  const root = createRoot(document.getElementById('root'));
  root.render(<App />);
})
