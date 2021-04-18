
import cv2
import numpy as np


img = np.random.randn(640, 480)
cv2.imshow("test", img)
if cv2.waitKey(0):
    cv2.destroyAllWindows()


