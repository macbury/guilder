import React from 'react';
import { useSignedIn } from '../../store/hooks/session';

export default function GuestGuard({ children }) {
  const isSignedIn = useSignedIn();

  if (isSignedIn) {
    return null
  }

  return children
}
