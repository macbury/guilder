import React, { useState, useCallback, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import Breadcrumb from 'rsuite/Breadcrumb';
import Col from 'rsuite/Col';
import Loader from 'rsuite/Loader';

import { Container,  MarginRow } from '../../components/TablePage';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useUpdateWalletMutation, Wallet, useGetWalletQuery } from '../../store/api';
import { showSuccess } from '../../toasts';
import WalletForm from './WalletForm';

export default function EditWalletPage() {
  const { walletId } = useParams();
  const navigate = useNavigate();
  const [value, setValue] = useState<Omit<Wallet, 'id'>>(null);
  const { isLoading: isLoadingWallet, data: walletData } = useGetWalletQuery(walletId);
  const [updateWallet, { data, isLoading, reset }] = useUpdateWalletMutation();

  useWindowTitle(isLoadingWallet ? `Edit wallet: ${walletData?.wallet?.name}` : "Loading wallet");

  useEffect(() => {
    if (walletData) {
      setValue(walletData)
    }
  }, [walletData])


  const submit = useCallback(async () => {
    const { data: { success } } = await updateWallet({ id: walletId, wallet: value });
    if (success) {
      reset();
      showSuccess("Updated wallet!");
      navigate("/wallets");
    }
  }, [updateWallet, walletId, value])

  const errors = data?.errors || {}

  if (isLoadingWallet) {
    return <Loader size="lg" backdrop content="Loading..." vertical />
  }

  return (
    <Container>
      <MarginRow>
        <Col xs={24}>
          <Breadcrumb>
            <Breadcrumb.Item href="/wallets">Wallets</Breadcrumb.Item>
            <Breadcrumb.Item active>Edit wallet: {walletData?.wallet?.name}</Breadcrumb.Item>
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
