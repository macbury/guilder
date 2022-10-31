import React from 'react';
import {
  Routes as Router,
  Route
} from "react-router-dom";

import SignInPage from "./pages/SignInPage";
import GuestGuard from "./components/authentication/GuestGuard";
import SignedInGuard from "./components/authentication/SignedInGuard";
import SearchAssetsPage from './pages/SearchAssetsPage';
import ShowAssetPage from './pages/Assets/ShowAssetPage';
import ListAssetsPage from './pages/Assets/ListAssetsPage';
import ListCategoriesPage from './pages/Categories/ListCategoriesPage';
import ListAccountsPage from './pages/Accounts/ListAccountsPage';
import ListIntegrationsPage from './pages/Integrations/ListIntegrationsPage';
import ListBondsPage from './pages/Bonds/ListBondsPage';
import ShowBondsPage from './pages/Bonds/ShowBondsPage';
import DashboardPage from './pages/Dashboard/DashboardPage';
import ListWalletsPage from './pages/Wallets/ListWalletsPage';
import NewWalletPage from './pages/Wallets/NewWalletPage';
import EditWalletPage from './pages/Wallets/EditWalletPage';

export default function Routes() {
  return (
    <React.Fragment>
      <GuestGuard>
        <SignInPage />
      </GuestGuard>
      <SignedInGuard>
        <Router>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/search" element={<SearchAssetsPage />} />
          <Route path="/search/:query" element={<SearchAssetsPage />} />
          <Route path="/assets" element={<ListAssetsPage />} />
          <Route path="/assets/:ticker/*" element={<ShowAssetPage />} />
          <Route path="/categories" element={<ListCategoriesPage />} />
          <Route path="/accounts" element={<ListAccountsPage />} />
          <Route path="/bonds/:id/*" element={<ShowBondsPage />} />
          <Route path="/bonds" element={<ListBondsPage />} />
          <Route path="/wallets" element={<ListWalletsPage />} />
          <Route path="/wallets/:walletId/edit" element={<EditWalletPage />} />
          <Route path="/wallets/new" element={<NewWalletPage />} />
          <Route path="/integrations" element={<ListIntegrationsPage />} />
        </Router>
      </SignedInGuard>
    </React.Fragment>
  )
}
