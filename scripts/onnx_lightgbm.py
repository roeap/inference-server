from pathlib import Path

import numpy as np
import onnxruntime
import torch
from hummingbird.ml import convert
from lightgbm.sklearn import LGBMRegressor
from sklearn import datasets

x, y = datasets.load_wine(return_X_y=True)
x = x.astype(np.float32)

model = LGBMRegressor(n_estimators=10)
model.fit(x, y)
preds = model.predict(x)

pytorch_model = convert(model, "pytorch")

result_dir = Path(__file__).absolute().parents[1] / "inference-server" / "tests" / "data" / "model.onnx"

torch.onnx.export(
    pytorch_model.model,
    (torch.from_numpy(x)),
    str(result_dir),
    input_names=["input"],
    output_names=["output"],
    dynamic_axes={"input": {0: "batch"}, "output": {0: "batch"}},
)

# sanity check - onnxruntime inference

sess = onnxruntime.InferenceSession(str(result_dir))
outputs = sess.run(None, {"input": x[:1]})[0][:, 0]

assert np.allclose(outputs, preds[:1])  # noqa
