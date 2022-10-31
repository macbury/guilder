import React from 'react';
import toaster from 'rsuite/toaster';
import Message from 'rsuite/Message';

export const showSuccess = (message : string) => {
  toaster.push(<Message closable type="success">{message}</Message>, { placement: "bottomEnd" })
};

export const showError = (message : string) => {
  toaster.push(<Message closable type="error">{message}</Message>, { placement: "bottomEnd" })
};
