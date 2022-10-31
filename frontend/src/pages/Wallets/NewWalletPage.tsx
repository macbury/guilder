import React, { useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import Breadcrumb from 'rsuite/Breadcrumb';
import Col from 'rsuite/Col';

import { Container,  MarginRow } from '../../components/TablePage';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useCreateWalletMutation, Wallet } from '../../store/api';
import { showSuccess } from '../../toasts';
import WalletForm from './WalletForm';

const INITIAL_VALUE = { name: "", currency: "", description: "" }

export default function NewWalletPage() {
  useWindowTitle("New Wallet");

  const navigate = useNavigate();
  const [value, setValue] = useState<Omit<Wallet, 'id'>>(INITIAL_VALUE);
  const [createWallet, { data, isLoading, reset }] = useCreateWalletMutation();

  const submit = useCallback(async () => {
    const { data: { success } } = await createWallet(value);
    if (success) {
      reset();
      setValue(INITIAL_VALUE);
      showSuccess("Created new wallet!");
      navigate("/wallets");
    }
  }, [createWallet, value])

  const errors = data?.errors || {}

  return (
    <Container>
      <MarginRow>
        <Col xs={24}>
          <Breadcrumb>
            <Breadcrumb.Item href="/wallets">Wallets</Breadcrumb.Item>
            <Breadcrumb.Item active>Setup new wallet</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
      </MarginRow>
      <WalletForm
        loading={isLoading}
        errors={errors}
        setValue={setValue}
        submit={submit}
        value={value}
      />
    </Container>
  )
}
