import {
  component$,
  useResource$,
  Resource,
  useSignal,
} from "@builder.io/qwik";

export default component$(() => {
  const signal = useSignal(0);

  const testDataResource = useResource$<string[]>(({ cleanup, track }) => {
    track(() => signal.value);

    const controller = new AbortController();
    cleanup(() => controller.abort());

    return getTestData(controller);
  });

  return (
    <div>
      <h1>
        Welcome to Qwik <span class="lightning">⚡️</span>
      </h1>
      {}
      <button onClick$={() => signal.value++}>Test {signal.value}</button>

      <Resource
        value={testDataResource}
        onPending={() => <h1>Loading...</h1>}
        onRejected={(error) => <>Error: {error.message}</>}
        onResolved={(data) => (
          <ul>
            {data.map((data) => (
              <li>{data}</li>
            ))}
          </ul>
        )}
      />
    </div>
  );
});

export async function getTestData(
  controller?: AbortController
): Promise<string[]> {
  const resp = await fetch("http://localhost:4000/test", {
    signal: controller?.signal,
  });

  const json = await resp.json();

  return Array.isArray(json) ? json : Promise.reject(json);
}
