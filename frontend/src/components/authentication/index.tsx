import React, { useCallback, useState, useEffect } from 'react';
import Loader from 'rsuite/Loader';
import { useAuthenticationManager } from '../../store/hooks/session';

export default function AuthenticationManager({ children }) {
  const {
    status, refresh
  } = useAuthenticationManager();

  useEffect(() => { refresh() }, [])

  if (status == 'initializing') {
    return <Loader size="lg" backdrop content="Authenticating..." vertical />
  }

  return children
}
