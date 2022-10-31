import React, { useCallback } from 'react';
import toaster from 'rsuite/toaster';
import Message from 'rsuite/Message';

export default function useToaster() {
  const showSuccess = useCallback((message : string) => {
    toaster.push(<Message closable type="success">{message}</Message>, { placement: "bottomEnd" })
  }, []);

  const showError = useCallback((message : string) => {
    toaster.push(<Message closable type="error">{message}</Message>, { placement: "bottomEnd" })
  }, []);

  return {
    showSuccess,
    showError
  }
}
