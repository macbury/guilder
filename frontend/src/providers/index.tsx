import React from 'react';
import ServerEventsObserver from "./ServerEventsObserver";
import SignedInGuard from '../components/authentication/SignedInGuard';

export default function Providers({ children }) {
  return (
    <React.Fragment>
      <SignedInGuard>
        <ServerEventsObserver />
      </SignedInGuard>
      {children}
    </React.Fragment>
  )
}
