import React, { useEffect } from 'react';
import { useAssetsState } from '../store/hooks/assets';
import { useIntegrationsState } from '../store/hooks/integrations';

export default function ServerEventsObserver() {
  const {
    actions: { updateAssets }
  } = useAssetsState();

  const {
    actions: { setIntegration }
  } = useIntegrationsState();

  useEffect(() => {
    const events = new EventSource("/api/live");

    events.addEventListener("message", (ev) => {
      const { integration, assets } = JSON.parse(ev.data);

      if (integration) {
        setIntegration(integration);
      }

      if (assets) {
        updateAssets(assets);
      }
    });

    return () => events.close()
  }, [])

  return null
}
