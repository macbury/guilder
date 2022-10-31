import React from 'react';
import { useSignedIn } from '../../store/hooks/session';

export default function SignedInGuard({ children }) {
  const isSignedIn = useSignedIn();

  if (isSignedIn) {
    return children
  }

  return null
}
